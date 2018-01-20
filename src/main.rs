#[macro_use]
extern crate quicksilver;

use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use std::time::Duration;

mod enemy;
use enemy::Enemy;
mod projectile;
use projectile::Projectile;

const PLAYER_RADIUS: i32 = 24;
const PLAYER_SPEED: f32 = 5.0;

pub struct State {
    pub player_pos: Circle,
    window: Window,
    canvas: Canvas,
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Hellevator", 960, 540);
        let player_pos = Circle::newi(100, 100, PLAYER_RADIUS);
        let enemies = vec![Enemy::new(Circle::newi(400, 400, PLAYER_RADIUS/2)), Enemy::new(Circle::newi(300, 400, PLAYER_RADIUS/2)), Enemy::new(Circle::newi(200, 250, PLAYER_RADIUS/2))];
        let projectiles = vec![];
        State { window, canvas, player_pos, enemies, projectiles }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        {
            let keyboard = self.window.keyboard();
            self.player_pos.x += if keyboard[Key::D].is_down() { PLAYER_SPEED } else { 0.0 };
            self.player_pos.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED } else { 0.0 };
            self.player_pos.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED } else { 0.0 };
            self.player_pos.y += if keyboard[Key::S].is_down() { PLAYER_SPEED } else { 0.0 };
            if keyboard[Key::Space].is_down() {
                self.projectiles.push(Projectile::new(Circle::newv(self.player_pos.center(), (PLAYER_RADIUS/8) as f32)));
            }
        }
        for e in self.enemies.iter_mut() {
            e.update(self.player_pos);
        }
        for p in self.projectiles.iter_mut() {
            p.update();
        }
        for p in self.projectiles.iter_mut() {
            for e in self.enemies.iter_mut() {
                if p.pos.overlaps_circ(e.pos) {
                    e.remove = true;
                    p.remove = true;
                }
            }
        }
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].remove {
                self.enemies.remove(i);
            } else {
                i += 1;
            }
        }
        i = 0;
        while i < self.projectiles.len() {
            if self.projectiles[i].remove {
                self.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
        while self.enemies.len() < 4 {
            self.enemies.push(Enemy::new(Circle::newi(0, 0, PLAYER_RADIUS/2)));
        }
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::black());
        self.canvas.draw_circle(self.player_pos, Color::white());
        for e in self.enemies.iter() {
            self.canvas.draw_circle(e.pos, Color::red());
        }
        for p in self.projectiles.iter() {
            self.canvas.draw_circle(p.pos, Color::yellow());
        }
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
