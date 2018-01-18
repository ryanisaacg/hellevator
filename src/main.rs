#[macro_use]
extern crate quicksilver;

use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use std::time::Duration;

const PLAYER_RADIUS: f32 = 48.0;
const PLAYER_SPEED: f32 = 5.0;

pub struct State {
    window: Window,
    canvas: Canvas,
    player_pos: Vector
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Hellevator", 960, 540);
        let player_pos = Vector::newi(100, 100);
        State { window, canvas, player_pos }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        let keyboard = self.window.keyboard();
        self.player_pos.x += if keyboard[Key::D].is_down() { PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::S].is_down() { PLAYER_SPEED } else { 0.0 };
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::black());
        self.canvas.draw_rect(Rectangle::newv(self.player_pos, Vector::new(PLAYER_RADIUS, PLAYER_RADIUS)), Color::white());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
