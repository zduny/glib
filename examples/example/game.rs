use std::f32::consts::PI;

use glium::{Display, Surface};
use glium::glutin::event::{Event, VirtualKeyCode};
use glium::glutin::window::Fullscreen;

use include_dir::*;

use glib::ProgramsCache;
use glib::input::{VirtualGamepad, VirtualGamepadStick, KeyboardMouseVirtualGamepad};
use glib::controllers::FirstPersonController;
use glib::math::Camera;
use glib::rendering::{Drawable, scene::{Scene, Transformable}};
use glib::fps::FpsCounter;
use glib::game::{EventHandler, GameEvent, KeyboardEvent, MouseEvent};
use glib::math::*;

pub struct Game {
    input: KeyboardMouseVirtualGamepad,
    controller: FirstPersonController,
    camera: Camera,
    scene: Scene,
    fps_counter: FpsCounter,
    should_quit_now: bool,
}

impl Game {
    pub fn new(display: &Display) -> Game {
        let mut input = KeyboardMouseVirtualGamepad::new();
        input.bind_default();

        let mut controller = FirstPersonController::new();
        let camera =
            Camera::perspective_for_display(display, controller.get_transform(), PI * 0.333);

        let chunks_directory = include_dir!("examples/example/shaders/chunks");
        let materials_directory = include_dir!("examples/example/shaders/materials");

        let programs_cache =
            ProgramsCache::new(display, "330", &chunks_directory, &materials_directory);
        let mut scene = Scene::new();
        scene.set_background_color(hex("#87ceeb"));
        //let test_cube = TestCube::new(display, &programs_cache);
        //scene.add(test_cube.get_mesh());
        let translation = Transform::translation(0.0, -0.05, -3.0) * Transform::scale(100.0, 100.0, 100.0);
        scene.set_transform(translation);

        let fps_counter = FpsCounter::new(1.0);

        let should_quit_now = false;
        Game {
            input,
            controller,
            camera,
            scene,
            fps_counter,
            should_quit_now,
        }
    }
}

impl EventHandler for Game {
    fn handle_game_event(&mut self, display: &Display, event: &mut GameEvent) {
        match event {
            GameEvent::Resize { .. } => {
                self.camera = Camera::perspective_for_display(display, IDENTITY, PI * 0.333);
            }
            GameEvent::Update {
                delta_time,
                stop_loop,
                ..
            } => {
                if self.should_quit_now {
                    *stop_loop = true;
                    return;
                }
                self.fps_counter.next_frame(*delta_time);
                display.gl_window().window().set_title(&format!(
                    "Glib Test - {:.0} FPS",
                    self.fps_counter.get_fps()
                ));

                let movement = self.input.stick_value(VirtualGamepadStick::Left) * (*delta_time as f32) * 5.0;
                self.controller
                    .change_position(dis3(movement.vector.x, 0.0, -movement.vector.y));
                let look_around = self.input.stick_value(VirtualGamepadStick::Right) * (*delta_time as f32);
                self.controller.yaw_and_pitch(look_around);
                
                self.camera.transform = self.controller.get_transform();
                self.input.on_after_update();
            }
            GameEvent::Draw => {
                let mut frame = display.draw();
                frame.clear_color(0.0, 0.0, 0.0, 0.0);
                self.scene
                    .draw(&mut frame, &IDENTITY, &self.camera)
                    .unwrap();
                frame.finish().unwrap();
            }
            _ => (),
        }
    }

    fn handle_keyboard_event(&mut self, display: &Display, event: KeyboardEvent) {
        match event {
            KeyboardEvent::KeyDown { input, .. } => {
                if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
                    //self.test_cube.toggle();
                }
            }
            KeyboardEvent::KeyUp { input, modifiers } => {
                if let Some(virtual_key_code) = input.virtual_keycode {
                    match virtual_key_code {
                        VirtualKeyCode::Return => {
                            if modifiers.alt() {
                                let gl_window = display.gl_window();
                                let window = gl_window.window();

                                if let Some(_fullscreen) = window.fullscreen() {
                                    window.set_fullscreen(None);
                                } else {
                                    let current_monitor = window.current_monitor();
                                    window.set_fullscreen(Some(Fullscreen::Borderless(
                                        current_monitor,
                                    )));
                                }
                            }
                        }
                        VirtualKeyCode::Escape => self.should_quit_now = true,
                        _ => (),
                    }
                }
            }
        }

        self.input.on_keyboard_event(event);
    }

    fn handle_mouse_event(&mut self, _display: &Display, event: MouseEvent) {
        self.input.on_mouse_event(event);
    }

    fn handle_raw(&mut self, _event: &Event<()>, _handled: &mut bool) {}
}