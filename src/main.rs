#[macro_use]
extern crate quicksilver;

use quicksilver::asset::*;
use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use quicksilver::*;

mod enemy;
use enemy::Enemy;

const PLAYER_RADIUS: i32 = 24;
const PLAYER_SPEED: f32 = 5.0;

pub struct LoadingScreen {
    player: LoadingAsset<Image>,
    crosshair: LoadingAsset<Image>
}

impl InitialScreen for LoadingScreen {
    fn configure() -> (Window, Canvas) {
        WindowBuilder::new().build("Hellevator", 960, 540)
    }

    fn new() -> Self {
        LoadingScreen {
            player: Image::load("img/ah_stand.png"),
            crosshair: Image::load("img/crosshair.png")
        }
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        if let LoadingAsset::Loaded(ref player_image) = self.player {
            if let LoadingAsset::Loaded(ref crosshair) = self.crosshair {
                let player_image = player_image.clone();
                let crosshair = crosshair.clone();
                let player_pos = Circle::newi(100, 100, PLAYER_RADIUS);
                let enemies = vec![Enemy::new(Circle::newi(400, 400, PLAYER_RADIUS/2)), 
                                   Enemy::new(Circle::newi(300, 400, PLAYER_RADIUS/2)), 
                                   Enemy::new(Circle::newi(200, 250, PLAYER_RADIUS/2))];
                Some(Box::new(GameScreen { player_pos, enemies, player_image, crosshair }))
            } else {
                self.crosshair.update();
                None
            }
        } else {
            self.player.update();
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.present(window);
    }
}

pub struct GameScreen {
    player_pos: Circle,
    enemies: Vec<Enemy>,
    player_image: Image,
    crosshair: Image
}

impl Screen for GameScreen {
    fn update(&mut self, window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        let keyboard = window.keyboard();
        self.player_pos.x += if keyboard[Key::D].is_down() { PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::S].is_down() { PLAYER_SPEED } else { 0.0 };
        for e in self.enemies.iter() {
            e.update();
        }
        None
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        canvas.draw_circle(self.player_pos, Color::white());
        for e in self.enemies.iter() {
            canvas.draw_circle(e.pos, Color::red());
        }
        canvas.present(window);
    }

}

screens_loop!(LoadingScreen);
