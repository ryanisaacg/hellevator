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

#[allow(unused_imports)]
use rand::Rng;

mod assets;
use assets::*;
mod enemy;
use enemy::*;
mod particle;
use particle::Particle;
mod projectile;
use projectile::*;
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
         StateMachine::Loading(Assets::load())
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
            *self = StateMachine::Game(GameScreen::new(Assets::new(loaded)));
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
