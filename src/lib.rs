use std::path::Path;
use std::{fmt::Display, collections::HashSet, rc::Rc, cell::RefCell};

use rand::seq::IteratorRandom;
use te_renderer::model::ModelVertex;
use te_renderer::state::GpuState;
use te_renderer::state::State as TeState;

mod display;
mod models;

pub struct Board {
    tiles: Vec<Vec<Tile>>,
    decision_tree: Vec<u8>, // TODO: u8 is placeholder // TODO: generate the decision tree
    width: u32,
    height: u32
}

impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let mut rows = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Tile::new(MaybeTile::Undecided(TileKind::all())))
            }
            rows.push(row)
        }

        Board {
            tiles: rows,
            decision_tree: vec![],
            width,
            height,
        }
    }

    pub fn clean(&mut self) {
        self.tiles = vec![];
        for _ in 0..self.height {
            let mut row = vec![];
            for _ in 0..self.width {
                row.push(Tile::new(MaybeTile::Undecided(TileKind::all())))
            }
            self.tiles.push(row)
        }
    }

    pub fn generate(&mut self) {
        while !self.finished() {
            self.generate_1()
        }
    }

    pub fn generate_1(&mut self) {
        let (row, col) = self.get_undecided();
        let tile = self.make_decision(row, col);
        self.propagate(tile, row, col);
    }

    pub fn generate_n(&mut self, n: u32) {
        for _ in 0..n {
            self.generate_1()
        }        
    }

    pub fn finished(&self) -> bool {
        !self.tiles.iter() // not(any undecided)
            .any(|row| row.iter().any(|t| match &t.kind {
                MaybeTile::Undecided(a) => a.len() > 0, // TODO: Once the decision tree is complete, change this to true (or maybe instead of boolean return an enum with 3 options, 1 for which there is undecided but all lens are 0)
                MaybeTile::Decided(_) => false,
            }))
    }

    fn get_undecided(&self) -> (usize, usize) {
        let mut rng = rand::thread_rng();

        self.tiles.iter().enumerate()
            .flat_map(|(i, row)| row.iter()
                .enumerate()
                .filter_map(move |(j, tile)| match &tile.kind {
                    MaybeTile::Undecided(a) => if a.len() == 0 {
                        None
                    } else {
                        Some((i, j))
                    },
                    MaybeTile::Decided(_) => None,
                })
            ).choose(&mut rng).unwrap()
    }

    fn make_decision(&mut self, row: usize, col: usize) -> TileKind {
        let mut rng = rand::thread_rng();
        let choice = match &self.tiles[row][col].kind {
            MaybeTile::Undecided(possibilities) => *possibilities.iter().choose(&mut rng).unwrap(),
            MaybeTile::Decided(_) => unreachable!(),
        };
        self.tiles[row][col] = Tile::new(MaybeTile::Decided(choice));
        choice
    }

    fn propagate(&mut self, tile: TileKind, row: usize, col: usize) {
        let mut prop_dir = |direction| {
            match (row, col).neighbour(direction, self.width, self.height) {
                Ok((row, col)) => {
                    match &mut self.tiles[row][col].kind {
                        MaybeTile::Undecided(possibilities) => {
                            tile.propagate(possibilities, direction);
                            if possibilities.len() == 1 {
                                let remaining_tile = *possibilities.iter().next().unwrap();
                                self.tiles[row][col] = Tile::new(MaybeTile::Decided(remaining_tile));
                                self.propagate(remaining_tile, row, col)
                            }
                        },
                        MaybeTile::Decided(_) => (),
                    }
                },
                Err(_) => ()
            }
        };

        prop_dir(Direction::North);
        prop_dir(Direction::East);
        prop_dir(Direction::South);
        prop_dir(Direction::West);
    }

    pub fn draw(&self, gpu: &GpuState, te_state: &mut TeState) {
        for (i, row) in self.tiles.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                match tile.kind {
                    MaybeTile::Undecided(_) => (),
                    MaybeTile::Decided(tile) => {
                        te_state.instances.place_custom_model(&tile.get_name(), gpu, (j as f32,0.0,i as f32), None);
                    },
                }
            }
        }
    }

    pub fn load_models(&self, gpu: &GpuState, te_state: &mut TeState) {
        let name = String::from("water");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("ground");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("tree");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("house");
        let model = get_model(gpu, te_state, name.clone(), models::HOUSE_V.into(), models::HOUSE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("road");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("hut");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("mountain");
        let model = get_model(gpu, te_state, name.clone(), models::MOUNTAIN_V.into(), models::MOUNTAIN_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));

        let name = String::from("sand");
        let model = get_model(gpu, te_state, name.clone(), models::SQUARE_V.into(), models::SQUARE_I.into());
        te_state.instances.place_custom_model(&name, gpu, (-1000.0,0.0,0.0), Some(model));
    }
}

fn get_model(gpu: &GpuState, te_state: &mut TeState, name: String, vertices: Vec<ModelVertex>, indices: Vec<u32>) -> te_renderer::model::Model {
    let image_path = Path::new("ignore").join("resources").join(format!("{name}.png"));
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

struct CoordError;

trait Coord: Sized{
    fn neighbour(&self, direction: Direction, width: u32, height: u32) -> Result<Self, CoordError>;
}

// 0, 0 is top-left
// 00 01
// 10 11
impl Coord for (usize, usize) {
    fn neighbour(&self, direction: Direction, width: u32, height: u32) -> Result<Self, CoordError> {
        match direction {
            Direction::North => match self.0 {
                0 => Err(CoordError),
                _ => Ok((self.0-1, self.1))
            },
            Direction::East => match self.1 {
                x if x+1 >= width as usize => Err(CoordError),
                _ => Ok((self.0, self.1+1))
            },
            Direction::South => match self.0 {
                y if y+1 >= height as usize => Err(CoordError),
                _ => Ok((self.0+1, self.1))
            },
            Direction::West => match self.1 {
                0 => Err(CoordError),
                _ => Ok((self.0, self.1-1))
            },
        }
    }
}

struct Tile {
    kind: MaybeTile
}

impl Tile {
    fn new(kind: MaybeTile) -> Tile {
        Tile { kind }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn opposite(&self, dir: &Direction) -> bool {
        match self {
            Direction::North => match dir {
                Direction::South => true,
                _ => false
            },
            Direction::East => match dir {
                Direction::West => true,
                _ => false
            },
            Direction::South => match dir {
                Direction::North => true,
                _ => false
            },
            Direction::West => match dir {
                Direction::East => true,
                _ => false
            },
        }
    }
}

#[derive(Debug)]
enum MaybeTile {
    Undecided(HashSet<TileKind>),
    Decided(TileKind)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum TileKind {
    Water,
    Ground,
    Tree,
    House(Direction),
    Road,
    Hut,
    Mountain,
    Sand
}

impl TileKind {
    fn all() -> HashSet<Self> {
        let mut set = HashSet::new();

        set.insert(TileKind::Water);
        set.insert(TileKind::Ground);
        set.insert(TileKind::Tree);
        set.insert(TileKind::House(Direction::North));
        set.insert(TileKind::House(Direction::East));
        set.insert(TileKind::House(Direction::South));
        set.insert(TileKind::House(Direction::West));
        set.insert(TileKind::Road);
        set.insert(TileKind::Hut);
        set.insert(TileKind::Mountain);
        set.insert(TileKind::Sand);
        set
    }

    fn get_name(&self) -> String {
        String::from(match self {
            TileKind::Water => "water",
            TileKind::Ground => "ground",
            TileKind::Tree => "tree",
            TileKind::House(_) => "house",
            TileKind::Road => "road",
            TileKind::Hut => "hut",
            TileKind::Mountain => "mountain",
            TileKind::Sand => "sand",
        }) 
    }

    fn propagate(&self, possibilities: &mut HashSet<TileKind>, direction: Direction) {
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

    fn get_rules(&self) -> Box<dyn Fn(&TileKind, Direction) -> bool + '_> {
        // Direction is this where "tile" is from "self"
        match self {
            TileKind::Water => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Water => true,
                TileKind::Sand => true,
                _ => false
            }),
            TileKind::Ground => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Water => false,
                TileKind::House(dir) => direction.opposite(dir),
                _ => true
            }),
            TileKind::Tree => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Ground => true,
                TileKind::Tree => true,
                TileKind::Hut => true,
                TileKind::Mountain => true,
                _ => false
            }),
            TileKind::House(dir) => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Ground => *dir != direction,
                TileKind::House(dir2) => *dir != direction && !direction.opposite(dir2),
                TileKind::Road => true,
                TileKind::Mountain => (*dir).opposite(&direction),
                _ => false
            }),
            TileKind::Road => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Ground => true,
                TileKind::House(_) => true,
                TileKind::Road => true,
                TileKind::Sand => true,
                _ => false
            }),
            TileKind::Hut => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Tree => true,
                TileKind::Mountain => true,
                _ => false
            }),
            TileKind::Mountain => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Ground => true,
                TileKind::Tree => true,
                TileKind::House(dir) => *dir == direction,
                TileKind::Hut => true,
                TileKind::Mountain => true,
                _ => false
            }),
            TileKind::Sand => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Water => true,
                TileKind::Ground => true,
                TileKind::Tree => true,
                TileKind::Road => true,
                TileKind::Sand => true,
                _ => false
            }),
        }
    }
}
