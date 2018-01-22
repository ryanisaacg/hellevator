#[macro_use]
extern crate quicksilver;

use quicksilver::asset::*;
use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::*;
use quicksilver::sound::*;
use quicksilver::*;

mod enemy;
use enemy::Enemy;
mod projectile;
use projectile::Projectile;
mod loading;
use loading::LoadingScreen;
mod game;
use game::*;

const PLAYER_RADIUS: i32 = 24;
const PLAYER_SPEED: f32 = 5.0;

screens_loop!(LoadingScreen);
