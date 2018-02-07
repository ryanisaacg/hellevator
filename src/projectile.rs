use super::*;

pub struct Projectile {
    pub pos: Circle,
    pub vel: Vector,
    pub remove: bool
}

impl Projectile {
    pub fn new(pos: Circle, vel: Vector) -> Projectile {
        Projectile { pos, vel, remove: false }
    }

    pub fn update(&mut self) {
        self.pos = self.pos.translate(self.vel);
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
