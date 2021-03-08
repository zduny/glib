mod game;

use std::sync::Arc;
use std::sync::Mutex;

use glium::{Display, glutin::{ContextBuilder, dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder}};
use glib::game::GameLoop;

use game::Game;

fn main() {
    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_title("Glib Test")
        .with_inner_size(LogicalSize::new(800.0, 600.0));

    let cb = ContextBuilder::new().with_vsync(true);

    let display = Display::new(wb, cb, &event_loop).unwrap();

    let game = Game::new(&display);
    event_loop.run_game_loop(vec![display], Arc::new(Mutex::new(game)));
}
