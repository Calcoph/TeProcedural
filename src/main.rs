use std::{rc::Rc, cell::RefCell};

use procedural::Board;
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
    state.board.generate();
    println!("{}", state.board);
    state.draw_board();
    let event_handler = move |event: Event<ControllerEvent>| {
        match event {
            te_player::event_loop::Event::NewEvents(_) => (),
            te_player::event_loop::Event::WindowEvent { event, .. } => {
                state.te_state.borrow_mut().input(&event);
                match event {
                    te_player::te_winit::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                        te_player::te_winit::event::ElementState::Pressed => match input.virtual_keycode.unwrap() {
                            te_player::te_winit::event::VirtualKeyCode::U => {
                                state.board.clean();
                                state.board.generate();
                                println!("{}", state.board);
                                state.draw_board()
                            },
                            _ => ()
                        },
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
    board: Board,
    gpu: Rc<RefCell<GpuState>>,
    te_state: Rc<RefCell<TeState>>
}

impl State {
    fn new(gpu: Rc<RefCell<GpuState>>, te_state: Rc<RefCell<TeState>>) -> State {
        State {
            board: Board::new(35, 10),
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