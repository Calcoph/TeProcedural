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
    deciding_coord: (usize, usize),
    tried_tiles: HashSet<T>,
    dead_ends: Vec<(usize, usize)>,
    temp_decided_coords: HashSet<(usize, usize)>
}

impl<T> DecisionBranch<T>
where
    T: Tile
{
    fn new(row: usize, col: usize) -> DecisionBranch<T> {
        DecisionBranch {
            deciding_coord: (row, col),
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
    pub tiles: Vec<Vec<MaybeTile<T>>>,
    decision_stack: Vec<DecisionBranch<T>>, // TODO: generate the decision tree
    dead_ends: Vec<(usize, usize)>,
    width: u32,
    height: u32
}

impl<T> Board<T>
where
    T: Tile
{
    pub fn new(width: u32, height: u32) -> Board<T> {
        let mut rows = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(MaybeTile::Undecided(T::all()))
            }
            rows.push(row)
        }

        let board = Board {
            tiles: rows,
            decision_stack: vec![],
            dead_ends: vec![],
            width,
            height,
        };

        board
    }

    pub fn clean(&mut self) {
        self.tiles = vec![];
        for _ in 0..self.height {
            let mut row = vec![];
            for _ in 0..self.width {
                row.push(MaybeTile::Undecided(T::all()))
            }
            self.tiles.push(row)
        }
        self.decision_stack = vec![];
        self.dead_ends = vec![];
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
                let (tile, row, col) = if self.can_continue_branch() {
                    if let Some(current_branch) = self.decision_stack.last_mut() {
                        let (row, col) = current_branch.deciding_coord;
                        let mut rng = rand::thread_rng();
                        let choice = match &self.tiles[row][col] {
                            MaybeTile::Undecided(possibilities) => *possibilities
                                .iter()
                                .filter(|tile| !current_branch.tried_tiles.contains(tile))
                                .choose(&mut rng)
                                .unwrap(),
                            MaybeTile::Decided(_) => unreachable!(),
                        };
                        self.tiles[row][col] = MaybeTile::Decided(choice);
                        current_branch.tried_tiles.insert(choice);
                        (choice, row, col)
                    } else {
                        unreachable!()
                    }
                } else {
                    let (row, col) = self.get_undecided();
                    let tile = self.make_decision(row, col);
                    let mut new_branch = DecisionBranch::new(row, col);
                    new_branch.tried_tiles.insert(tile);
                    self.decision_stack.push(new_branch);
                    (tile, row, col)
                };
                let new_propagated = self.propagate(tile, row, col);
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
        let undecideds_left = self.tiles.iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(_) => true,
            MaybeTile::Decided(_) => false,
        }));

        let impossibilities = self.tiles.iter().any(|v| v.iter().any(|t| match t {
            MaybeTile::Undecided(possibilities) => possibilities.len() == 0,
            MaybeTile::Decided(_) => false,
        }));

        let branch_end = self.tiles.iter().enumerate().all(|(i, v)| v.iter().enumerate().all(|(j, t)| match t {
            MaybeTile::Undecided(possibilities) => {
                if possibilities.len() > 0 {
                    if let Some(current_branch) = self.decision_stack.last() {
                        if current_branch.deciding_coord == (i, j) {
                            if possibilities.len() <= current_branch.tried_tiles.len() {
                                true
                            } else {
                                false
                            }
                        } else if current_branch.dead_ends.contains(&(i, j)) {
                            true
                        } else {
                            false
                        }
                    } else {
                        if self.dead_ends.contains(&(i, j)) {
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
        }));

        match (impossibilities, branch_end, undecideds_left) {
            (true, _, _) => BranchStatus::DeadEnd,
            (_, true, true) => BranchStatus::DeadEnd,
            (_, _, false) => BranchStatus::Complete,
            (_, _, _) => BranchStatus::Incomplete,
        }
    }

    fn get_undecided(&self) -> (usize, usize) {
        let options = self.tiles.iter().enumerate()
            .flat_map(|(i, row)| row.iter()
                .enumerate()
                .filter_map(move |(j, tile)| match tile {
                    MaybeTile::Undecided(a) => if a.len() == 0 {
                        None
                    } else if let Some(current_branch) = self.decision_stack.last() {
                        if current_branch.dead_ends.contains(&(i, j)) {
                            None
                        } else {
                            Some(((i, j), a.len()))
                        }
                    } else {
                        if self.dead_ends.contains(&(i, j)) {
                            None
                        } else {
                            Some(((i, j), a.len()))    
                        }
                    },
                    MaybeTile::Decided(_) => None,
                })
            );
        match options.clone().find(|option| option.1 == 1) {
            Some(best_option) => best_option.0,
            None => {
                let mut rng = rand::thread_rng();
                let option = options.choose(&mut rng).unwrap().0;
                option
            },
        }
    }

    fn make_decision(&mut self, row: usize, col: usize) -> T {
        let mut rng = rand::thread_rng();
        let choice = match &self.tiles[row][col] {
            MaybeTile::Undecided(possibilities) => *possibilities.iter().choose(&mut rng).unwrap(),
            MaybeTile::Decided(_) => unreachable!(),
        };
        self.tiles[row][col] = MaybeTile::Decided(choice);
        choice
    }

    fn propagate(&mut self, tile: T, row: usize, col: usize) -> Vec<(usize, usize)>{
        let mut v = vec![];
        let mut prop_dir = |direction: T::Direction, v: &mut Vec<(usize, usize)>| {
            match direction.neighbour(row, col, self.width, self.height) {
                Ok((row, col)) => {
                    match &mut self.tiles[row][col] {
                        MaybeTile::Undecided(possibilities) => {
                            v.push((row, col));
                            tile.propagate(possibilities, direction);
                            if possibilities.len() == 1 {
                                let remaining_tile = *possibilities.iter().next().unwrap();
                                self.tiles[row][col] = MaybeTile::Decided(remaining_tile);
                                v.extend(self.propagate(remaining_tile, row, col).into_iter());
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

    #[cfg(feature = "view3d")]
    pub fn draw(&self, gpu: &GpuState, te_state: &mut TeState) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match tile {
                    MaybeTile::Undecided(_) => (),
                    MaybeTile::Decided(tile) => {
                        te_state.instances.place_custom_model(&tile.get_name(), gpu, (j as f32,0.0,i as f32), None);
                    },
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
            let (row, col) = current_branch.deciding_coord;
            self.tiles[row][col] = MaybeTile::Undecided(T::all());
            for (row, col) in current_branch.temp_decided_coords.iter() {
                self.tiles[*row][*col] = MaybeTile::Undecided(T::all());
            };
            self.recalculate(row, col);
            for (row, col) in current_branch.temp_decided_coords.iter() {
                self.recalculate(*row, *col);
            };
            if let Some(previous_branch) = self.decision_stack.last_mut() {
                previous_branch.dead_ends.push(current_branch.deciding_coord)
            } else{
                self.dead_ends.push(current_branch.deciding_coord)
            }
            Ok(())
        } else {
            Err(ImpossibleBoardError)
        }
    }

    fn can_continue_branch(&self) -> bool {
        if let Some(current_branch) = self.decision_stack.last() {
            let (row, col) = current_branch.deciding_coord;
            match &self.tiles[row][col] {
                MaybeTile::Undecided(_) => true,
                MaybeTile::Decided(_) => false,
            }
        } else {
            false
        }
    }

    fn recalculate(&mut self, row: usize, col: usize) {
        let mut recalc = |direction: T::Direction| {
            match direction.neighbour(row, col, self.width, self.height) {
                Ok((row_t, col_t)) => {
                    match self.tiles[row_t][col_t] {
                        MaybeTile::Undecided(_) => (),
                        MaybeTile::Decided(tile) => {
                            let possibilities = match &mut self.tiles[row][col] {
                                MaybeTile::Undecided(possibilities) => possibilities,
                                MaybeTile::Decided(_) => unreachable!(),
                            };
                            tile.propagate(possibilities, direction.opposite());
                            if possibilities.len() == 1 {
                                self.tiles[row_t][col_t] = MaybeTile::Decided(*possibilities.iter().next().unwrap())
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

pub trait Direction: Sized + Copy {
    fn all() -> Vec<Self>;
    fn neighbour(&self, row: usize, col: usize, width: u32, height: u32) -> Result<(usize, usize), CoordError>;
    fn opposite(&self) -> Self;
}

#[derive(Debug)]
pub enum MaybeTile<T>
where
    T: Tile,
{
    Undecided(HashSet<T>),
    Decided(T),
}

pub trait Tile: Sized + Eq + PartialEq + Hash + Clone + Copy {
    type Direction: Direction;

    fn all() -> HashSet<Self>;
    #[cfg(feature = "view3d")]
    fn get_name(&self) -> String;
    #[cfg(feature = "view3d")]
    fn get_model(&self) -> Option<(Vec<ModelVertex>, Vec<u32>)>;
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
