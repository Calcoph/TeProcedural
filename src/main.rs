use std::collections::HashSet;
use std::fmt::Display;
use std::{rc::Rc, cell::RefCell};

use colored::Colorize;

use procedural::{Board, Direction, Tile};
use te_gamepad::gamepad::ControllerEvent;
use te_player::event_loop::{self, Event};
use te_renderer::{initial_config::InitialConfiguration, state::GpuState};
use te_renderer::state::State as TeState;

fn main() {
    let (event_loop, gpu, window, te_state) = pollster::block_on(te_player::prepare(InitialConfiguration {
        resource_files_directory: String::from("ignore/resources"),
        map_files_directory: String::from("ignore"),
        font_dir_path: String::from("ignore"),
        default_texture_path: String::from("ignore"),
        icon_path: String::from("icon.png"),
        camera_sensitivity: 2.0,
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

struct State {
    board: Board<TileKind>,
    gpu: Rc<RefCell<GpuState>>,
    te_state: Rc<RefCell<TeState>>
}

impl State {
    fn new(gpu: Rc<RefCell<GpuState>>, te_state: Rc<RefCell<TeState>>) -> State {
        let file = std::fs::read_to_string("size.txt").unwrap();
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

impl Tile for TileKind {
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
            TileKind::Water => Box::new(|tile: &TileKind, _direction: Direction| match tile {
                TileKind::Water => true,
                TileKind::Sand => true,
                _ => false
            }),
            TileKind::Ground => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Water => false,
                TileKind::House(dir) => direction.is_opposite(dir),
                TileKind::Hut => false,
                _ => true
            }),
            TileKind::Tree => Box::new(|tile: &TileKind, _direction: Direction| match tile {
                TileKind::Ground => true,
                TileKind::Tree => true,
                TileKind::Hut => true,
                TileKind::Mountain => true,
                TileKind::Sand => true,
                _ => false
            }),
            TileKind::House(dir) => Box::new(|tile: &TileKind, direction: Direction| match tile {
                TileKind::Ground => *dir != direction,
                TileKind::House(dir2) => *dir != direction && !direction.is_opposite(dir2),
                TileKind::Road => true,
                TileKind::Mountain => (*dir).is_opposite(&direction),
                _ => false
            }),
            TileKind::Road => Box::new(|tile: &TileKind, _direction: Direction| match tile {
                TileKind::Ground => true,
                TileKind::House(_) => true,
                TileKind::Road => true,
                TileKind::Sand => true,
                _ => false
            }),
            TileKind::Hut => Box::new(|tile: &TileKind, _direction: Direction| match tile {
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
            TileKind::Sand => Box::new(|tile: &TileKind, _direction: Direction| match tile {
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

impl Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character = match self {
            TileKind::Water => "~".blue(),
            TileKind::Ground => "O".bold().on_green(),
            TileKind::Tree => "B".green(),
            TileKind::House(dir) => match dir {
                Direction::North => "#".magenta().on_blue(),
                Direction::East => "#".magenta().on_green(),
                Direction::South => "#".magenta(),
                Direction::West => "#".magenta().on_yellow(),
            },
            TileKind::Road => "-".purple(),
            TileKind::Hut => "v".red(),
            TileKind::Mountain => "X".on_red(),
            TileKind::Sand => "~".yellow(),
        };
        write!(f, "{}", character)
    }
}
