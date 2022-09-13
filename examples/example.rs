use std::collections::HashSet;
use std::fmt::Display;
#[cfg(feature = "view3d")]
use std::{rc::Rc, cell::RefCell};

use colored::Colorize;

use procedural::{Board, Tile, Direction, CoordError};
#[cfg(feature = "view3d")]
use te_gamepad::gamepad::ControllerEvent;
#[cfg(feature = "view3d")]
use te_player::event_loop::{self, Event};
#[cfg(feature = "view3d")]
use te_renderer::{initial_config::InitialConfiguration, state::GpuState};
#[cfg(feature = "view3d")]
use te_renderer::state::State as TeState;

#[cfg(feature = "view3d")]
mod models;

#[cfg(feature = "view3d")]
fn main() {
    let (event_loop, gpu, window, te_state) = pollster::block_on(te_player::prepare(InitialConfiguration {
        font_dir_path: String::from("resources/font"),
        icon_path: String::from("resources/icon.png"),
        camera_sensitivity: 2.0,
        window_name: String::from("procedural"),
        screen_height: 500,
        screen_width: 1000,
        ..Default::default()
    }, false)).unwrap();

    let mut state = State::new(gpu.clone(), te_state.clone());
    state.te_state.borrow_mut().instances.place_sprite("white.png", &state.gpu.borrow(), Some((265.0, 100.0)), (1.0, 1.0, 0.0));
    let text = "Move with WASD
Move up/down with spacebar/shift
Pan with Q/E
Look up/down with Z/X
Zoom in/out with R/F
Generate a new map with U";
    let lines = text.split("\n")
        .map(|s| s.chars().map(|c| match c {
            'A'..='Z' => c.to_lowercase().to_string() + "+",
            'a'..='z' => c.to_string() + "+",
            ' ' => String::from("space"),
            '/' => String::from("slash"),
            _ => unimplemented!()
        }).collect()).enumerate();
    for (height, line) in lines {
        state.te_state.borrow_mut().instances.place_text(line, &gpu.borrow(), None, (5.0, 15.0*height as f32+5.0, 1.0));
    }
    match state.board.generate() {
        Ok(_) => {
            state.draw_board();
        },
        Err(_) => (),
    };
    state.board.clean();
    let event_handler = move |event: Event<ControllerEvent>| {
        match event {
            te_player::event_loop::Event::WindowEvent { event, .. } => {
                state.te_state.borrow_mut().input(&event);
                match event {
                    te_player::te_winit::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        te_player::te_winit::event::ElementState::Pressed => if let Some(val) = input.virtual_keycode { match val {
                            te_player::te_winit::event::VirtualKeyCode::U => {
                                state.board.clean();
                                match state.board.generate() {
                                    Ok(_) => {
                                        state.draw_board()
                                    },
                                    Err(_) => (),
                                };
                            },
                            _ => ()
                        }},
                        te_player::te_winit::event::ElementState::Released => (),
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    };
    let event_handler = Box::new(event_handler);
    event_loop::run(event_loop, window, gpu, te_state, event_handler);
}

#[cfg(not(feature = "view3d"))]
fn main() {
    let file = std::fs::read_to_string("resources/size.txt").unwrap();
    let mut file = file.split("\n");
    let width = u32::from_str_radix(file.next().unwrap(), 10).unwrap();
    let height = u32::from_str_radix(file.next().unwrap(), 10).unwrap();
    println!("Initializing board");
    let mut board: Board<ExampleTile> = Board::new(width, height);
    println!("{}", board);
    println!("Here is the first tile");
    board.generate_1().unwrap();
    println!("{}", board);
    println!("Generating the entire board");
    match board.generate() {
        Ok(_) => {
            println!("Board successfully generated");
            println!("{}", board);
        },
        Err(_) => {
            println!("It is (probably) impossible to make a board of this size with this set of rules");
            println!("Here is a valid (but unfinished) board:");
            println!("{}", board);
        },
    }
}
#[cfg(feature = "view3d")]
struct State {
    board: Board<ExampleTile>,
    gpu: Rc<RefCell<GpuState>>,
    te_state: Rc<RefCell<TeState>>
}

#[cfg(feature = "view3d")]
impl State {
    fn new(gpu: Rc<RefCell<GpuState>>, te_state: Rc<RefCell<TeState>>) -> State {
        let file = std::fs::read_to_string("resources/size.txt").unwrap();
        let mut file = file.split("\n");
        let width = u32::from_str_radix(file.next().unwrap(), 10).unwrap();
        let height = u32::from_str_radix(file.next().unwrap(), 10).unwrap();
        State {
            board: Board::new(width, height),
            gpu,
            te_state,
        }
    }

    fn draw_board(&mut self) {
        self.te_state.borrow_mut().instances.forget_all_3d_instances();
        self.board.load_models(&self.gpu.borrow(), &mut self.te_state.borrow_mut());
        self.board.draw(&self.gpu.borrow(), &mut self.te_state.borrow_mut());
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum ExampleTile {
    Water,
    Ground,
    Tree,
    House(Direction4),
    Road,
    Hut,
    Mountain,
    Sand
}

impl Tile for ExampleTile {
    type Direction = Direction4;

    fn all() -> HashSet<Self> {
        let mut set = HashSet::new();

        set.insert(ExampleTile::Water);
        set.insert(ExampleTile::Ground);
        set.insert(ExampleTile::Tree);
        set.insert(ExampleTile::House(Direction4::North));
        set.insert(ExampleTile::House(Direction4::East));
        set.insert(ExampleTile::House(Direction4::South));
        set.insert(ExampleTile::House(Direction4::West));
        set.insert(ExampleTile::Road);
        set.insert(ExampleTile::Hut);
        set.insert(ExampleTile::Mountain);
        set.insert(ExampleTile::Sand);
        set
    }

    #[cfg(feature = "view3d")]
    fn get_name(&self) -> String {
        String::from(match self {
            ExampleTile::Water => "water",
            ExampleTile::Ground => "ground",
            ExampleTile::Tree => "tree",
            ExampleTile::House(_) => "house",
            ExampleTile::Road => "road",
            ExampleTile::Hut => "hut",
            ExampleTile::Mountain => "mountain",
            ExampleTile::Sand => "sand",
        }) 
    }

    fn get_rules(&self) -> Box<dyn Fn(&ExampleTile, Direction4) -> bool + '_> {
        // Direction is where "tile" is from "self"
        match self {
            // Water can only be next to water or sand
            ExampleTile::Water => Box::new(|tile: &ExampleTile, _direction: Direction4| match tile {
                ExampleTile::Water => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Ground can be next to anything, except: in front of a house, water, hut.
            ExampleTile::Ground => Box::new(|tile: &ExampleTile, direction: Direction4| match tile {
                ExampleTile::Water => false,
                ExampleTile::House(dir) => direction.is_opposite(dir),
                ExampleTile::Hut => false,
                _ => true
            }),
            // Trees can only be next to ground, trees, huts, mountains and sand
            ExampleTile::Tree => Box::new(|tile: &ExampleTile, _direction: Direction4| match tile {
                ExampleTile::Ground => true,
                ExampleTile::Tree => true,
                ExampleTile::Hut => true,
                ExampleTile::Mountain => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Only roads are allowed to be in front of houses. They can have houses or ground or more roads around them. They can also have mountains behind them
            ExampleTile::House(dir) => Box::new(|tile: &ExampleTile, direction: Direction4| match tile {
                ExampleTile::Ground => *dir != direction,
                ExampleTile::House(dir2) => *dir != direction && !direction.is_opposite(dir2),
                ExampleTile::Road => true,
                ExampleTile::Mountain => (*dir).is_opposite(&direction),
                _ => false
            }),
            // Roads can be next to ground, houses, roads and sand
            ExampleTile::Road => Box::new(|tile: &ExampleTile, _direction: Direction4| match tile {
                ExampleTile::Ground => true,
                ExampleTile::House(_) => true,
                ExampleTile::Road => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Huts can only be next to trees and mountains
            ExampleTile::Hut => Box::new(|tile: &ExampleTile, _direction: Direction4| match tile {
                ExampleTile::Tree => true,
                ExampleTile::Mountain => true,
                _ => false
            }),
            // Mountains can be next to ground, trees, huts and mountains. They can also be behind houses.
            ExampleTile::Mountain => Box::new(|tile: &ExampleTile, direction: Direction4| match tile {
                ExampleTile::Ground => true,
                ExampleTile::Tree => true,
                ExampleTile::House(dir) => *dir == direction,
                ExampleTile::Hut => true,
                ExampleTile::Mountain => true,
                _ => false
            }),
            // Sand can be next to water, ground, trees, roads and sand
            ExampleTile::Sand => Box::new(|tile: &ExampleTile, _direction: Direction4| match tile {
                ExampleTile::Water => true,
                ExampleTile::Ground => true,
                ExampleTile::Tree => true,
                ExampleTile::Road => true,
                ExampleTile::Sand => true,
                _ => false
            }),
        }
    }

    #[cfg(feature = "view3d")]
    fn get_model(&self) -> Option<(Vec<te_renderer::model::ModelVertex>, Vec<u32>)> {
        match self {
            ExampleTile::Water => Some((models::SQUARE_V.into(), models::SQUARE_I.into())),
            ExampleTile::Ground => Some((models::SQUARE_V.into(), models::SQUARE_I.into())),
            ExampleTile::Tree => Some((models::TREE_V.into(), models::TREE_I.into())),
            ExampleTile::House(_) => Some((models::HOUSE_V.into(), models::HOUSE_I.into())),
            ExampleTile::Road => Some((models::SQUARE_V.into(), models::SQUARE_I.into())),
            ExampleTile::Hut => Some((models::SQUARE_V.into(), models::SQUARE_I.into())),
            ExampleTile::Mountain => Some((models::MOUNTAIN_V.into(), models::MOUNTAIN_I.into())),
            ExampleTile::Sand => Some((models::SQUARE_V.into(), models::SQUARE_I.into())),
        }
    }
}

impl Display for ExampleTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            ExampleTile::Water => "~".blue(),
            ExampleTile::Ground => "O".bold().on_green(),
            ExampleTile::Tree => "B".green(),
            ExampleTile::House(dir) => match dir {
                Direction4::North => "#".magenta().on_blue(),
                Direction4::East => "#".magenta().on_green(),
                Direction4::South => "#".magenta(),
                Direction4::West => "#".magenta().on_yellow(),
            },
            ExampleTile::Road => "-".purple(),
            ExampleTile::Hut => "v".red(),
            ExampleTile::Mountain => "X".on_red(),
            ExampleTile::Sand => "~".yellow(),
        };
        write!(f, "{}", character)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction4 {
    North,
    East,
    South,
    West
}

impl Direction4 {
    pub fn is_opposite(&self, dir: &Direction4) -> bool {
        match self {
            Direction4::North => match dir {
                Direction4::South => true,
                _ => false
            },
            Direction4::East => match dir {
                Direction4::West => true,
                _ => false
            },
            Direction4::South => match dir {
                Direction4::North => true,
                _ => false
            },
            Direction4::West => match dir {
                Direction4::East => true,
                _ => false
            },
        }
    }
}

impl Direction for Direction4 {
    fn all() -> Vec<Self> {
        vec![Direction4::North, Direction4::East, Direction4::South, Direction4::West]
    }

    fn neighbour(&self, row: usize, col: usize, width: u32, height: u32) -> Result<(usize, usize), procedural::CoordError> {
        match self {
            Direction4::North => match row {
                0 => Err(CoordError),
                _ => Ok((row-1, col))
            },
            Direction4::East => match col {
                x if x+1 >= width as usize => Err(CoordError),
                _ => Ok((row, col+1))
            },
            Direction4::South => match row {
                y if y+1 >= height as usize => Err(CoordError),
                _ => Ok((row+1, col))
            },
            Direction4::West => match col {
                0 => Err(CoordError),
                _ => Ok((row, col-1))
            },
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction4::North => Direction4::South,
            Direction4::East => Direction4::West,
            Direction4::South => Direction4::North,
            Direction4::West => Direction4::East,
        }
    }
}