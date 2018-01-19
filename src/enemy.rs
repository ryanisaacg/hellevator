use quicksilver::geom::*;

pub struct Enemy {
    pub pos: Circle
}

impl Enemy {
    pub fn new(pos: Circle) -> Enemy {
        Enemy { pos }
    }

    pub fn update(&mut self, player: Circle) {
        self.pos = self.pos.translate((player.center() - self.pos.center()).normalize() * 4);
    }
}
