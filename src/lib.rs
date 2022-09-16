use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashSet};
#[cfg(feature = "view3d")]
use std::path::Path;

use rand::seq::IteratorRandom;
#[cfg(feature = "view3d")]
use te_renderer::model::ModelVertex;
#[cfg(feature = "view3d")]
use te_renderer::state::GpuState;
#[cfg(feature = "view3d")]
use te_renderer::state::State as TeState;

mod display;

#[derive(Debug)]
struct DecisionBranch<T>
where
    T: Tile
{
    deciding_coord: (usize, usize, usize),
    tried_tiles: HashSet<T>,
    dead_ends: Vec<(usize, usize, usize)>,
    temp_decided_coords: HashSet<(usize, usize, usize)>
}

impl<T> DecisionBranch<T>
where
    T: Tile
{
    fn new(row: usize, col: usize, layer: usize) -> DecisionBranch<T> {
        DecisionBranch {
            deciding_coord: (row, col, layer),
            tried_tiles: HashSet::new(),
            dead_ends: vec![],
            temp_decided_coords: HashSet::new()
        }
    }
}

pub enum BranchStatus {
    Complete,
    DeadEnd,
    Incomplete
}

#[derive(Debug)]
pub struct ImpossibleBoardError;

pub struct Board<T>
where
    T: Tile
{
    pub tiles: Vec<Vec<Vec<MaybeTile<T>>>>,
    decision_stack: Vec<DecisionBranch<T>>,
    dead_ends: Vec<(usize, usize, usize)>,
    current_layer: usize,
    width: u32,
    length: u32,
    height: u32
}

impl<T> Board<T>
where
    T: Tile
{
    pub fn new(width: u32, length: u32, height: u32) -> Board<T> {
        let mut layers = vec![];
        for cur_layer in 0..height as usize {
            let mut layer = vec![];
            for _ in 0..length {
                let mut row = vec![];
                for _ in 0..width {
                    row.push(MaybeTile::Undecided(T::possibles(cur_layer)))
                }
                layer.push(row)
            }
            layers.push(layer)
        }

        let board = Board {
            tiles: layers,
            decision_stack: vec![],
            dead_ends: vec![],
            width,
            length,
            height,
            current_layer: 0,
        };

        #[cfg(feature = "validate")]
        board.validate();

        board
    }

    #[cfg(feature = "validate")]
    fn validate(&self) {
        let all = T::all();
        let mut names = HashSet::new();
        all.iter().for_each(|tile| {
            if !names.insert(tile.get_name()) {
                println!("WARNING: There is more than one tile with the name `{}`", tile.get_name())
            }
        });
        let directions = T::Direction::all();
        all.iter()
            .flat_map(|tile| all.iter().map(|t| (tile, t)).collect::<Vec<_>>())
            .flat_map(|(tile1, tile2)| directions.iter().map(|direction| (tile1, tile2, direction)).collect::<Vec<_>>())
            .for_each(|(tile1, tile2, direction)| {
                if tile1.get_rules()(tile2, *direction)
                    !=
                    tile2.get_rules()(tile1, direction.opposite())
                {
                    println!("ERROR: Rules are not bidirectional for {tile1:?} {tile2:?} {direction:?}")
                }
            });
    }

    pub fn clean(&mut self) {
        self.tiles = vec![];
        for cur_layer in 0..self.height as usize {
            let mut layer = vec![];
            for _ in 0..self.length {
                let mut row = vec![];
                for _ in 0..self.width {
                    row.push(MaybeTile::Undecided(T::possibles(cur_layer)))
                }
                layer.push(row)
            }
            self.tiles.push(layer)
        }
        self.decision_stack = vec![];
        self.dead_ends = vec![];
        self.current_layer = 0;
    }

    pub fn generate(&mut self) -> Result<(), ImpossibleBoardError> {
        let mut complete = false;
        while !complete {
            complete = self.generate_1()?;
        };
        Ok(())
    }

    pub fn generate_1(&mut self) -> Result<bool, ImpossibleBoardError> {
        match self.get_status() {
            BranchStatus::Complete => Ok(true),
            BranchStatus::DeadEnd => {self.go_back()?;Ok(false)},
            BranchStatus::Incomplete => {
                let (tile, row, col, layer) = if self.can_continue_branch() {
                    if let Some(current_branch) = self.decision_stack.last_mut() {
                        let (row, col, layer) = current_branch.deciding_coord;
                        let mut rng = rand::thread_rng();
                        let choice = match &self.tiles[layer][row][col] {
                            MaybeTile::Undecided(possibilities) => *possibilities
                                .iter()
                                .filter(|tile| !current_branch.tried_tiles.contains(tile))
                                .choose(&mut rng)
                                .unwrap(),
                            MaybeTile::Decided(_) => unreachable!(),
                        };
                        self.tiles[layer][row][col] = MaybeTile::Decided(choice);
                        current_branch.tried_tiles.insert(choice);
                        (choice, row, col, layer)
                    } else {
                        unreachable!()
                    }
                } else {
                    let (row, col, layer) = self.get_undecided();
                    let tile = self.make_decision(row, col, layer);
                    let mut new_branch = DecisionBranch::new(row, col, layer);
                    new_branch.tried_tiles.insert(tile);
                    self.decision_stack.push(new_branch);
                    (tile, row, col, layer)
                };
                let new_propagated = self.propagate(tile, row, col, layer);
                let current_branch = self.decision_stack.last_mut().unwrap();
                current_branch.temp_decided_coords.extend(new_propagated.into_iter());
                Ok(false)
            }
        }
    }

    pub fn generate_n(&mut self, n: u32) -> Result<(), ImpossibleBoardError> {
        for _ in 0..n {
            if self.generate_1()? {
                break
            }
        };
        Ok(())
    }

    pub fn get_status(&self) -> BranchStatus {
        let undecideds_left = self.tiles.iter().any(|v| v.iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(_) => true,
            MaybeTile::Decided(_) => false,
        })));

        let impossibilities = self.tiles.iter().any(|v| v.iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(possibilities) => possibilities.len() == 0,
            MaybeTile::Decided(_) => false,
        })));

        let branch_end = self.tiles.iter().enumerate().all(|(k, v)| v.iter().enumerate().all(|(i, v)| v.iter().enumerate().all(|(j, t)| match t {
            MaybeTile::Undecided(possibilities) => {
                if possibilities.len() > 0 {
                    if let Some(current_branch) = self.decision_stack.last() {
                        if current_branch.deciding_coord == (i, j, k) {
                            if possibilities.len() <= current_branch.tried_tiles.len() {
                                true
                            } else {
                                false
                            }
                        } else if current_branch.dead_ends.contains(&(i, j, k)) {
                            true
                        } else {
                            false
                        }
                    } else {
                        if self.dead_ends.contains(&(i, j, k)) {
                            true
                        } else {
                            false
                        }
                    }
                } else {
                    true
                }
            },
            MaybeTile::Decided(_) => true,
        })));

        match (impossibilities, branch_end, undecideds_left) {
            (_, _, false) => BranchStatus::Complete,
            (true, _, _) => BranchStatus::DeadEnd,
            (_, true, _) => BranchStatus::DeadEnd,
            (_, _, _) => BranchStatus::Incomplete,
        }
    }

    fn get_undecided(&mut self) -> (usize, usize, usize) {
        let options: Vec<_>;
        loop {
            let layer = self.tiles.get(self.current_layer).unwrap();
            let opt = layer.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    let board = &self;
                    row.iter()
                        .enumerate()
                        .filter_map(move |(j, tile)| match tile {
                            MaybeTile::Undecided(a) => if a.len() == 0 {
                                None
                            } else if let Some(current_branch) = board.decision_stack.last() {
                                if current_branch.dead_ends.contains(&(i, j, board.current_layer)) {
                                    None
                                } else {
                                    Some(((i, j, board.current_layer), a.len()))
                                }
                            } else {
                                if board.dead_ends.contains(&(i, j, board.current_layer)) {
                                    None
                                } else {
                                    Some(((i, j, board.current_layer), a.len()))    
                                }
                            },
                            MaybeTile::Decided(_) => None,
                        })
                }).collect::<Vec<_>>();
                let min = match opt.iter().min_by_key(|(_, key)| key) {
                    Some((_, min)) => Some(*min),
                    None => None,
                };
                match min {
                    Some(min) => {
                        options = opt.into_iter().filter(|(_, a)| *a == min).collect();
                        break;
                    },
                    None => self.current_layer += 1, // This layer is done
                }
        }
        let mut rng = rand::thread_rng();
        let option = options.iter().choose(&mut rng).unwrap().0;
        option
    }

    fn make_decision(&mut self, row: usize, col: usize, layer: usize) -> T {
        let mut rng = rand::thread_rng();
        let choice = match &self.tiles[layer][row][col] {
            MaybeTile::Undecided(possibilities) => *possibilities.iter().choose(&mut rng).unwrap(),
            MaybeTile::Decided(_) => unreachable!(),
        };
        self.tiles[layer][row][col] = MaybeTile::Decided(choice);
        choice
    }

    fn propagate(&mut self, tile: T, row: usize, col: usize, layer: usize) -> Vec<(usize, usize, usize)> {
        let mut v = vec![];
        let mut prop_dir = |direction: T::Direction, v: &mut Vec<(usize, usize, usize)>| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => {
                    match &mut self.tiles[layer][row][col] {
                        MaybeTile::Undecided(possibilities) => {
                            v.push((row, col, layer));
                            let len = possibilities.len();
                            tile.propagate(possibilities, direction);
                            if possibilities.len() == 1 {
                                let remaining_tile = *possibilities.iter().next().unwrap();
                                self.tiles[layer][row][col] = MaybeTile::Decided(remaining_tile);
                                v.extend(self.propagate(remaining_tile, row, col, layer).into_iter());
                            } else if possibilities.len() < len {
                                let possibilities = possibilities.clone();
                                let changes = self.propagate_possibilities(&possibilities, row, col, layer);
                                for ((row, col, layer), new_possibilities) in changes {
                                    match &mut self.tiles[layer][row][col] {
                                        MaybeTile::Undecided(_) => {
                                            self.tiles[layer][row][col] = MaybeTile::Undecided(new_possibilities);
                                            v.push((row, col, layer))
                                        },
                                        MaybeTile::Decided(t) => {
                                            if !new_possibilities.contains(t) {
                                                unreachable!()
                                            }
                                        },
                                    }
                                }
                            }
                        },
                        MaybeTile::Decided(_) => (),
                    }
                },
                Err(_) => ()
            }
        };
        for direction in Direction::all() {
            prop_dir(direction, &mut v);
        }
        v
    }

    fn propagate_possibilities(&self, possibilities: &HashSet<T>, row: usize, col: usize, layer: usize) -> Vec<((usize, usize, usize), HashSet<T>)> {
        let mut v = vec![];
        let prop_dir = |tile: T, direction: T::Direction| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => {
                    match &self.tiles[layer][row][col] {
                        MaybeTile::Undecided(possibilities) => {
                            let mut new_possibilities = possibilities.clone();
                            tile.propagate(&mut new_possibilities, direction);

                            Some(new_possibilities)
                        },
                        MaybeTile::Decided(_) => None,
                    }
                },
                Err(_) => None
            }
        };
        for direction in T::Direction::all() {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row, col, layer)) => match &self.tiles[layer][row][col] {
                    MaybeTile::Undecided(next_possibilities) => {
                        let len = next_possibilities.len();
                        let mut next_possibilities = HashSet::new();
                        for tile in possibilities.iter() {
                            match prop_dir(*tile, direction) {
                                Some(h) => for t in h {
                                    next_possibilities.insert(t);
                                },
                                None => (),
                            };
                        }
                        if len != next_possibilities.len() {
                            v.push(((row, col, layer), next_possibilities))
                        }
                    },
                    MaybeTile::Decided(_) => (),
                },
                Err(_) => (),
            }
        }
        v
    }

    #[cfg(feature = "view3d")]
    pub fn draw(&self, gpu: &GpuState, te_state: &mut TeState) {
        for (k, layer) in self.tiles.iter().enumerate() {
            for (i, row) in layer.iter().enumerate() {
                for (j, tile) in row.iter().enumerate() {
                    match tile {
                        MaybeTile::Undecided(_) => (),
                        MaybeTile::Decided(tile) => {
                            if tile.has_model() {
                                te_state.instances.place_custom_model(&tile.get_name(), gpu, (j as f32,k as f32,i as f32), None);
                            }
                        },
                    }
                }
            }
        }
    }

    #[cfg(feature = "view3d")]
    pub fn load_models(&self, gpu: &GpuState, te_state: &mut TeState) {
        for tile in T::all() {
            match tile.get_model() {
                Some((vertices, indices)) => {
                    let name = tile.get_name();
                    let model = get_model(gpu, te_state, name.clone(), vertices, indices);
                    te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));            
                },
                None => (),
            }
        }
    }

    fn go_back(&mut self) -> Result<(), ImpossibleBoardError> {
        if let Some(current_branch) = self.decision_stack.pop() {
            let (row, col, layer) = current_branch.deciding_coord;
            self.tiles[layer][row][col] = MaybeTile::Undecided(T::possibles(layer));
            for (row, col, layer) in current_branch.temp_decided_coords.iter() {
                self.tiles[*layer][*row][*col] = MaybeTile::Undecided(T::possibles(*layer));
            };
            self.recalculate(row, col, layer);
            for (row, col, layer) in current_branch.temp_decided_coords.iter() {
                self.recalculate(*row, *col, *layer);
            };
            if let Some(previous_branch) = self.decision_stack.last_mut() {
                previous_branch.dead_ends.push(current_branch.deciding_coord);
                self.current_layer = previous_branch.deciding_coord.2;
            } else{
                self.current_layer = 0;
                self.dead_ends.push(current_branch.deciding_coord)
            }
            Ok(())
        } else {
            Err(ImpossibleBoardError)
        }
    }

    fn can_continue_branch(&self) -> bool {
        if let Some(current_branch) = self.decision_stack.last() {
            let (row, col, layer) = current_branch.deciding_coord;
            match &self.tiles[layer][row][col] {
                MaybeTile::Undecided(_) => true,
                MaybeTile::Decided(_) => false,
            }
        } else {
            false
        }
    }

    fn recalculate(&mut self, row: usize, col: usize, layer: usize) {
        let mut recalc = |direction: T::Direction| {
            match direction.neighbour(row, col, layer, self.width, self.length, self.height) {
                Ok((row_t, col_t, layer_t)) => {
                    match self.tiles[layer_t][row_t][col_t] {
                        MaybeTile::Undecided(_) => (),
                        MaybeTile::Decided(tile) => {
                            let possibilities = match &mut self.tiles[layer][row][col] {
                                MaybeTile::Undecided(possibilities) => possibilities,
                                MaybeTile::Decided(_) => unreachable!(),
                            };
                            tile.propagate(possibilities, direction.opposite());
                            if possibilities.len() == 1 {
                                self.tiles[layer_t][row_t][col_t] = MaybeTile::Decided(*possibilities.iter().next().unwrap())
                            }
                        },
                    }
                },
                Err(_) => ()
            }
        };
        for direction in Direction::all() {
            recalc(direction);
        }
    }
}

#[cfg(feature = "view3d")]
fn get_model(gpu: &GpuState, te_state: &mut TeState, name: String, vertices: Vec<ModelVertex>, indices: Vec<u32>) -> te_renderer::model::Model {
    let image_path = Path::new("resources").join("tiles").join(format!("{name}.png"));
    let img = image::open(image_path).unwrap();
    let img = img.as_rgba8().unwrap();
    let texture = te_renderer::texture::Texture::from_dyn_image(
        &gpu.device,
        &gpu.queue,
        &img,
        None
    ).unwrap();
    let materials = vec![te_renderer::model::Material::new(
        &gpu.device,
        "material",
        texture,
        &te_state.instances.layout
    )];
    let material_count = 0;
    let mesh = te_renderer::model::Mesh::new(
        name.clone(),
        &name,
        vertices,
        indices,
        material_count,
        &gpu.device
    );
    let meshes = vec![mesh];
    te_renderer::model::Model{ meshes, transparent_meshes: vec![], materials }
}

pub struct CoordError;

#[cfg(feature = "validate")]
pub trait Direction: Sized + Copy + Debug {
    direction!();
}

#[cfg(not(feature = "validate"))]
pub trait Direction: Sized + Copy {
    direction!();
}

#[macro_export]
macro_rules! direction {
    () => {
        fn all() -> Vec<Self>;
        fn neighbour(&self, row: usize, col: usize, layer: usize, width: u32, length: u32, height: u32) -> Result<(usize, usize, usize), CoordError>;
        fn opposite(&self) -> Self;
    }
}

#[derive(Debug)]
pub enum MaybeTile<T>
where
    T: Tile,
{
    Undecided(HashSet<T>),
    Decided(T),
}

#[cfg(feature = "validate")]
pub trait Tile: Sized + Eq + PartialEq + Hash + Clone + Copy + Debug {
    tile!();
}

#[cfg(not(feature = "validate"))]
pub trait Tile: Sized + Eq + PartialEq + Hash + Clone + Copy {
    tile!();
}

#[macro_export]
macro_rules! tile {
    () => {
        type Direction: Direction;

        fn all() -> HashSet<Self>;
        fn possibles(layer: usize) -> HashSet<Self>;
        #[cfg(feature = "view3d")]
        fn get_name(&self) -> String;
        #[cfg(feature = "view3d")]
        fn get_model(&self) -> Option<(Vec<ModelVertex>, Vec<u32>)>;
        fn has_model(&self) -> bool;
        fn propagate(&self, possibilities: &mut HashSet<Self>, direction: Self::Direction) {
            let can_stay = self.get_rules();
            let mut to_remove = vec![];
            for possibility in possibilities.iter() {
                if !can_stay(possibility, direction) {
                    to_remove.push(*possibility);
                }
            }
            for rem in to_remove {
                possibilities.remove(&rem);
            }
        }
        fn get_rules(&self) -> Box<dyn Fn(&Self, Self::Direction) -> bool + '_>;
    }
}