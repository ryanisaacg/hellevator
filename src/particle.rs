use super::*;

pub struct Particle {
    pub image: Image,
    pub pos: Vector,
    pub velocity: Vector,
    pub rotation: f32,
    pub rotational_velocity: f32,
    pub lifetime: i32
}

impl Killable for Particle {
    fn is_dead(&self) -> bool {
        self.lifetime <= 0
    }
}
