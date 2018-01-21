use quicksilver::geom::*;

pub struct Projectile {
    pub pos: Circle,
    pub remove: bool
}

impl Projectile {
    pub fn new(pos: Circle) -> Projectile {
        Projectile { pos, remove: false }
    }

    pub fn update(&mut self) {
        self.pos.x += 5.0;

        if self.pos.x > 1000.0 {
            self.remove = true;
        }
    }
}
