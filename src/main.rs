extern crate futures;
extern crate quicksilver;
extern crate rand;

use futures::{Async, Future};
use futures::future::{JoinAll, join_all};

use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use quicksilver::sound::*;
use quicksilver::*;

use rand::Rng;

mod enemy;
use enemy::Enemy;
use enemy::EnemyType;
mod projectile;
use projectile::Projectile;
use projectile::ProjectileType;
mod loading;
use loading::LoadingScreen;
mod game;
use game::*;

const PLAYER_RADIUS: i32 = 24; //Size of the player
const PLAYER_SPEED: f32 = 5.0; //Speed of the player
const CORD_HEALTH: f32 = 1000.0; //Max health of the cord

trait Killable {
    fn is_dead(&self) -> bool;
}

screens_loop!(LoadingScreen);
