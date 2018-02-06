#[macro_use]
extern crate quicksilver;
extern crate rand;

use quicksilver::asset::*;
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
mod loading;
use loading::LoadingScreen;
mod game;
use game::*;

const PLAYER_RADIUS: i32 = 24; //Size of the player
const PLAYER_SPEED: f32 = 5.0; //Speed of the player
const CORD_HEALTH: f32 = 1000.0; //Max health of the cord

screens_loop!(LoadingScreen);
