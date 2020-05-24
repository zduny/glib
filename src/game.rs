use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::*;

use glium::Display;

use glium::glutin::dpi::PhysicalSize;
use glium::glutin::event::DeviceEvent;
use glium::glutin::event::ElementState;
use glium::glutin::event::Event;
use glium::glutin::event::KeyboardInput;
use glium::glutin::event::ModifiersState;
use glium::glutin::event::MouseScrollDelta;
use glium::glutin::event::StartCause;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event_loop::EventLoop;

use crate::math::*;

pub enum KeyboardEvent {
    KeyDown {
        input: KeyboardInput,
        modifiers: ModifiersState,
    },
    KeyUp {
        input: KeyboardInput,
        modifiers: ModifiersState,
    },
}

pub enum MouseEvent {
    MouseMove {
        cursor_position: Option<Position2>,
        delta: Displacement2,
        modifiers: ModifiersState,
    },
    MouseDown {
        cursor_position: Option<Position2>,
        button: u32,
        modifiers: ModifiersState,
    },
    MouseUp {
        cursor_position: Option<Position2>,
        button: u32,
        modifiers: ModifiersState,
    },
    MouseWheel {
        cursor_position: Option<Position2>,
        delta: MouseScrollDelta,
        modifiers: ModifiersState,
    },
}

pub enum GameEvent {
    Resize {
        new_size: PhysicalSize<u32>,
    },
    Start,
    Update {
        delta_time: f64,
        stop_loop: bool,
        needs_redraw: bool,
        wait_for_events: bool,
    },
    Draw,
    Close {
        cancel: bool,
    },
}

pub trait EventHandler {
    fn handle_game_event(&mut self, display: &Display, event: &mut GameEvent);
    fn handle_keyboard_event(&mut self, display: &Display, event: KeyboardEvent);
    fn handle_mouse_event(&mut self, display: &Display, event: MouseEvent);
    fn handle_raw(&mut self, event: &Event<()>, handled: &mut bool);
}

pub trait GameLoop {
    fn run_game_loop<E: 'static + EventHandler>(
        self,
        displays: Vec<Display>,
        event_handler: Arc<Mutex<E>>,
    );
}

struct PerDisplayState {
    display: Display,
    modifiers_state: ModifiersState,
    cursor_position: Position2,
    cursor_over: bool,
}

impl GameLoop for EventLoop<()> {
    fn run_game_loop<E: 'static + EventHandler>(
        self,
        displays: Vec<Display>,
        event_handler: Arc<Mutex<E>>,
    ) {
        let mut displays_dictionary = HashMap::new();
        for display in displays {
            let window_id = display.gl_window().window().id();
            let state = PerDisplayState {
                display,
                modifiers_state: Default::default(),
                cursor_position: pos2(0.0, 0.0),
                cursor_over: false,
            };
            displays_dictionary.insert(window_id, state);
        }

        let mut now = Instant::now();
        self.run(move |event, _, control_flow| {
            use std::borrow::BorrowMut;
            let mut lock = event_handler.lock().unwrap();
            let event_handler = lock.borrow_mut();

            if displays_dictionary.is_empty() {
                *control_flow = ControlFlow::Exit;
            }

            let mut handled = false;
            event_handler.handle_raw(&event, &mut handled);
            if handled {
                return;
            }

            match event {
                Event::WindowEvent {
                    event: window_event,
                    window_id,
                } => {
                    if let Some(display_state) = displays_dictionary.get_mut(&window_id) {
                        match window_event {
                            WindowEvent::CloseRequested => {
                                let mut event = GameEvent::Close { cancel: false };
                                event_handler.handle_game_event(&display_state.display, &mut event);

                                if let GameEvent::Close { cancel } = event {
                                    if !cancel {
                                        displays_dictionary.remove(&window_id);
                                    }
                                }
                            }
                            WindowEvent::Resized(size) => {
                                let mut event = GameEvent::Resize { new_size: size };
                                event_handler.handle_game_event(&display_state.display, &mut event);
                            }
                            WindowEvent::ModifiersChanged(modifiers_state) => {
                                display_state.modifiers_state = modifiers_state;
                            }
                            WindowEvent::KeyboardInput { input, .. } => {
                                let event = if let ElementState::Pressed = input.state {
                                    KeyboardEvent::KeyDown {
                                        input,
                                        modifiers: display_state.modifiers_state,
                                    }
                                } else {
                                    KeyboardEvent::KeyUp {
                                        input,
                                        modifiers: display_state.modifiers_state,
                                    }
                                };
                                event_handler.handle_keyboard_event(&display_state.display, event);
                            }
                            WindowEvent::CursorEntered { .. } => {
                                display_state.cursor_over = true;
                            }
                            WindowEvent::CursorLeft { .. } => {
                                display_state.cursor_over = false;
                            }
                            WindowEvent::CursorMoved { position, .. } => {
                                display_state.cursor_position =
                                    pos2(position.x as f32, position.y as f32);
                            }
                            _ => (),
                        }
                        return;
                    } else {
                        return;
                    }
                }
                Event::DeviceEvent {
                    event: device_event,
                    ..
                } => match device_event {
                    DeviceEvent::MouseMotion { delta } => {
                        for display_state in displays_dictionary.values() {
                            let cursor_position = if display_state.cursor_over {
                                Some(display_state.cursor_position)
                            } else {
                                None
                            };
                            let event = MouseEvent::MouseMove {
                                cursor_position,
                                delta: dis2(delta.0 as f32, delta.1 as f32),
                                modifiers: display_state.modifiers_state,
                            };
                            event_handler.handle_mouse_event(&display_state.display, event);
                        }
                        return;
                    }
                    DeviceEvent::MouseWheel { delta } => {
                        for display_state in displays_dictionary.values() {
                            let cursor_position = if display_state.cursor_over {
                                Some(display_state.cursor_position)
                            } else {
                                None
                            };
                            let event = MouseEvent::MouseWheel {
                                cursor_position,
                                delta,
                                modifiers: display_state.modifiers_state,
                            };
                            event_handler.handle_mouse_event(&display_state.display, event);
                        }
                        return;
                    }
                    DeviceEvent::Button { button, state } => {
                        for display_state in displays_dictionary.values() {
                            let cursor_position = if display_state.cursor_over {
                                Some(display_state.cursor_position)
                            } else {
                                None
                            };
                            let event = match state {
                                ElementState::Pressed => MouseEvent::MouseDown {
                                    cursor_position,
                                    button,
                                    modifiers: display_state.modifiers_state,
                                },
                                ElementState::Released => MouseEvent::MouseUp {
                                    cursor_position,
                                    button,
                                    modifiers: display_state.modifiers_state,
                                },
                            };
                            event_handler.handle_mouse_event(&display_state.display, event);
                        }
                        return;
                    }
                    _ => return,
                },
                Event::NewEvents(cause) => match cause {
                    StartCause::Init => {
                        for display_state in displays_dictionary.values() {
                            let mut event = GameEvent::Start;
                            event_handler.handle_game_event(&display_state.display, &mut event);
                        }
                        return;
                    }
                    StartCause::Poll => (),
                    _ => return,
                },
                _ => return,
            }

            let delta_time = now.elapsed().as_secs_f64();
            now = Instant::now();
            let mut wait_instead_of_polling = true;
            displays_dictionary.retain(|_window_id, display_state| {
                let mut event = GameEvent::Update {
                    delta_time,
                    stop_loop: false,
                    needs_redraw: true,
                    wait_for_events: false,
                };
                event_handler.handle_game_event(&display_state.display, &mut event);
                if let GameEvent::Update {
                    stop_loop,
                    needs_redraw,
                    wait_for_events,
                    ..
                } = event
                {
                    if stop_loop {
                        false
                    } else {
                        if needs_redraw {
                            let mut event = GameEvent::Draw;
                            event_handler.handle_game_event(&display_state.display, &mut event);
                        }

                        wait_instead_of_polling = wait_instead_of_polling && wait_for_events;

                        true
                    }
                } else {
                    true
                }
            });

            if wait_instead_of_polling {
                *control_flow = ControlFlow::Wait;
            }
        });
    }
}
