use quicksilver::geom::*;

pub struct Enemy {
    pub pos: Circle
}

impl Enemy {
    pub fn new(pos: Circle) -> Enemy {
        Enemy { pos }
    }

    pub fn update(&self) {
        
    }
}
