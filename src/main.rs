extern crate futures;
#[macro_use] extern crate itertools;
#[macro_use] extern crate quicksilver;
extern crate rand;

use futures::{Async, Future};
use futures::future::{Join, JoinAll, join_all};

use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use quicksilver::sound::*;
use quicksilver::*;

use rand::Rng;

mod enemy;
use enemy::Enemy;
use enemy::EnemyType;
mod particle;
use particle::Particle;
mod projectile;
use projectile::Projectile;
use projectile::ProjectileType;
mod game;
use game::*;

const PLAYER_RADIUS: i32 = 24; //Size of the player
const PLAYER_SPEED: f32 = 5.0; //Speed of the player
const CORD_HEALTH: f32 = 1000.0; //Max health of the cord

trait Killable {
    fn is_dead(&self) -> bool;
}

type LoadingValue = Join<JoinAll<Vec<ImageLoader>>, JoinAll<Vec<SoundLoader>>>;

enum StateMachine {
    Loading(LoadingValue),
    Game(GameScreen)
}

impl State for StateMachine {
    fn configure() -> Window {
        WindowBuilder::new()
            .with_show_cursor(false)
            .build("Hellevator", 960, 540)
    }

    fn new() -> StateMachine {
         StateMachine::Loading(join_all(vec![
                Image::load("img/ah_stand.png"),
                Image::load("img/crosshair.png"),
                Image::load("img/gun.png"),
                Image::load("img/wood.png"),
                Image::load("img/shadow.png"),
                Image::load("img/wall.png"),
                Image::load("img/bat.png"),
                Image::load("img/md_stand.png"),
                Image::load("img/spider.png"),
                Image::load("img/angry_spider.png"),
                Image::load("img/gear.png"),
                Image::load("img/web_spider.png"),
                Image::load("img/spiderweb.png"),
                Image::load("img/explode_spider.png"),
                Image::load("img/mama_spider.png"),
                Image::load("img/plus.png"),
                Image::load("img/spider_skitter.png"),
                Image::load("img/wire.png")])
            .join(join_all(vec![
                Sound::load("snd/gun.wav"),
                Sound::load("snd/bat-death.wav")])))
    }

    fn update(&mut self, window: &mut Window) {
        let loaded_assets = if let &mut StateMachine::Loading(_) = self {
            match self {
                &mut StateMachine::Loading(ref mut future) => future.poll(),
                _ => unreachable!()
            }.unwrap()
        } else {
            Async::NotReady
        };
        if let Async::Ready(loaded) = loaded_assets {
            *self = StateMachine::Game(GameScreen::new(loaded));
        }
        if let &mut StateMachine::Game(ref mut state) = self {
            state.update(window);
        }
    }

    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        if let &mut StateMachine::Game(ref mut state) = self {
            state.draw(window);
        }
        window.present();
    }
}

run!(StateMachine);
