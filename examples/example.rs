use std::collections::HashSet;
use std::fmt::Display;
#[cfg(feature = "view3d")]
use std::{rc::Rc, cell::RefCell};

use colored::Colorize;

use procedural::{Board, Direction, Tile};
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
        resource_files_directory: String::from("ignore"),
        map_files_directory: String::from("ignore"),
        font_dir_path: String::from("ignore"),
        default_texture_path: String::from("ignore"),
        icon_path: String::from("resources/icon.png"),
        camera_sensitivity: 2.0,
        window_name: String::from("procedural"),
        ..Default::default()
    }, false)).unwrap();

    let mut state = State::new(gpu.clone(), te_state.clone());
    match state.board.generate() {
        Ok(_) => {
            state.draw_board();
        },
        Err(_) => (),
    };
    state.board.clean();
    let event_handler = move |event: Event<ControllerEvent>| {
        match event {
            te_player::event_loop::Event::NewEvents(_) => (),
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
            te_player::event_loop::Event::DeviceEvent { .. } => (),
            te_player::event_loop::Event::UserEvent(_) => (),
            te_player::event_loop::Event::Suspended => (),
            te_player::event_loop::Event::Resumed => (),
            te_player::event_loop::Event::MainEventsCleared => (),
            te_player::event_loop::Event::RedrawRequested(_) => (),
            te_player::event_loop::Event::RedrawEventsCleared => (),
            te_player::event_loop::Event::LoopDestroyed => (),
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
        self.te_state.borrow_mut().instances.forget_all_instances();
        self.board.load_models(&self.gpu.borrow(), &mut self.te_state.borrow_mut());
        self.board.draw(&self.gpu.borrow(), &mut self.te_state.borrow_mut());
    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum ExampleTile {
    Water,
    Ground,
    Tree,
    House(Direction),
    Road,
    Hut,
    Mountain,
    Sand
}

impl Tile for ExampleTile {
    fn all() -> HashSet<Self> {
        let mut set = HashSet::new();

        set.insert(ExampleTile::Water);
        set.insert(ExampleTile::Ground);
        set.insert(ExampleTile::Tree);
        set.insert(ExampleTile::House(Direction::North));
        set.insert(ExampleTile::House(Direction::East));
        set.insert(ExampleTile::House(Direction::South));
        set.insert(ExampleTile::House(Direction::West));
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

    fn get_rules(&self) -> Box<dyn Fn(&ExampleTile, Direction) -> bool + '_> {
        // Direction is where "tile" is from "self"
        match self {
            // Water can only be next to water or sand
            ExampleTile::Water => Box::new(|tile: &ExampleTile, _direction: Direction| match tile {
                ExampleTile::Water => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Ground can be next to anything, except: in front of a house, water, hut.
            ExampleTile::Ground => Box::new(|tile: &ExampleTile, direction: Direction| match tile {
                ExampleTile::Water => false,
                ExampleTile::House(dir) => direction.is_opposite(dir),
                ExampleTile::Hut => false,
                _ => true
            }),
            // Trees can only be next to ground, trees, huts, mountains and sand
            ExampleTile::Tree => Box::new(|tile: &ExampleTile, _direction: Direction| match tile {
                ExampleTile::Ground => true,
                ExampleTile::Tree => true,
                ExampleTile::Hut => true,
                ExampleTile::Mountain => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Only roads are allowed to be in front of houses. They can have houses or ground or more roads around them. They can also have mountains behind them
            ExampleTile::House(dir) => Box::new(|tile: &ExampleTile, direction: Direction| match tile {
                ExampleTile::Ground => *dir != direction,
                ExampleTile::House(dir2) => *dir != direction && !direction.is_opposite(dir2),
                ExampleTile::Road => true,
                ExampleTile::Mountain => (*dir).is_opposite(&direction),
                _ => false
            }),
            // Roads can be next to ground, houses, roads and sand
            ExampleTile::Road => Box::new(|tile: &ExampleTile, _direction: Direction| match tile {
                ExampleTile::Ground => true,
                ExampleTile::House(_) => true,
                ExampleTile::Road => true,
                ExampleTile::Sand => true,
                _ => false
            }),
            // Huts can only be next to trees and mountains
            ExampleTile::Hut => Box::new(|tile: &ExampleTile, _direction: Direction| match tile {
                ExampleTile::Tree => true,
                ExampleTile::Mountain => true,
                _ => false
            }),
            // Mountains can be next to ground, trees, huts and mountains. They can also be behind houses.
            ExampleTile::Mountain => Box::new(|tile: &ExampleTile, direction: Direction| match tile {
                ExampleTile::Ground => true,
                ExampleTile::Tree => true,
                ExampleTile::House(dir) => *dir == direction,
                ExampleTile::Hut => true,
                ExampleTile::Mountain => true,
                _ => false
            }),
            // Sand can be next to water, ground, trees, roads and sand
            ExampleTile::Sand => Box::new(|tile: &ExampleTile, _direction: Direction| match tile {
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
                Direction::North => "#".magenta().on_blue(),
                Direction::East => "#".magenta().on_green(),
                Direction::South => "#".magenta(),
                Direction::West => "#".magenta().on_yellow(),
            },
            ExampleTile::Road => "-".purple(),
            ExampleTile::Hut => "v".red(),
            ExampleTile::Mountain => "X".on_red(),
            ExampleTile::Sand => "~".yellow(),
        };
        write!(f, "{}", character)
    }
}
