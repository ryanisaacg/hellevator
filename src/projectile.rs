use super::*;

#[derive(Copy, Clone, PartialEq)]
pub enum ProjectileType {
    PlayerBullet,
    EnemyBullet,
    Web(i32)
}

pub struct Projectile {
    pub pos: Circle,
    pub vel: Vector,
    pub proj_type: ProjectileType,
    pub remove: bool
}

impl Projectile {
    pub fn new(pos: Circle, vel: Vector, proj_type: ProjectileType) -> Projectile {
        Projectile { pos, vel, proj_type, remove: false }
    }

    pub fn update(&mut self) {
        match(self.proj_type) {
            ProjectileType::PlayerBullet | ProjectileType::EnemyBullet => {
                self.pos = self.pos.translate(self.vel);
            },
            ProjectileType::Web(ref mut timer) => {
                *timer += 1;
                if *timer < 90 {
                    self.pos = self.pos.translate(self.vel);
                } else if *timer < 210 {
                    self.pos.radius = 50.0;
                } else {
                    self.remove = true;
                }
            }
        }
        if self.pos.x < -100.0 || self.pos.x > 1100.0 || self.pos.y < -100.0 || self.pos.y > 1100.0 {
            self.remove = true;
        }
    }
}

impl Killable for Projectile {
    fn is_dead(&self) -> bool {
        self.remove
    }
}
