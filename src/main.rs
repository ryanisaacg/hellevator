#[macro_use]
extern crate quicksilver;

use quicksilver::geom::*;
use quicksilver::graphics::*;
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Hellevator", 960, 540);
        State { window, canvas }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::black());
        self.canvas.draw_rect(Rectangle::newi_sized(32, 32), Color::white());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
