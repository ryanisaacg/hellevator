use quicksilver::geom::*;

pub struct Enemy {
    pub pos: Circle,
    pub remove: bool
}

impl Enemy {
    pub fn new(pos: Circle) -> Enemy {
        Enemy { pos, remove: false }
    }

    pub fn update(&mut self, player: Circle, cord_pos: Circle, cord_health: &mut f32) {
        self.pos = self.pos.translate((cord_pos.center() - self.pos.center()).normalize() * 4);

        if self.pos.overlaps_circ(cord_pos) {
            self.remove = true;
            *cord_health -= 10.0;
        }
    }
}
