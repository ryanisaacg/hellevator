use quicksilver::geom::*;

pub struct Enemy {
    pub pos: Circle,
    pub remove: bool
}

impl Enemy {
    pub fn new(pos: Circle) -> Enemy {
        Enemy { pos, remove: false }
    }

    pub fn update(&mut self, player: Circle) {
        self.pos = self.pos.translate((player.center() - self.pos.center()).normalize() * 4);

        if (self.pos.center() - player.center()).len2() < 100.0 {
            self.remove = true;
        }
    }
}
