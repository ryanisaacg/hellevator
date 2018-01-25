use super::*;

pub enum EnemyType {
    Bat,
    Gunner
}

pub struct Enemy {
    pub pos: Circle,
    pub enemy_type: EnemyType,
    pub remove: bool
}

impl Enemy {
    pub fn new(pos: Circle, enemy_type: EnemyType) -> Enemy {
        Enemy { pos, enemy_type, remove: false }
    }

    pub fn update(&mut self, player: Circle, cord_pos: Circle, cord_health: &mut f32, enemy_projectiles: &mut Vec<Projectile>) {
        match self.enemy_type {
            EnemyType::Bat => {
                self.pos = self.pos.translate((cord_pos.center() - self.pos.center()).normalize() * 2);

                if self.pos.overlaps_circ(cord_pos) {
                    self.remove = true;
                    *cord_health -= 10.0;
                }
            },
            EnemyType::Gunner => {
                if (self.pos.center() - player.center()).len2() > 500.0*500.0 {
                    self.pos = self.pos.translate((player.center() - self.pos.center()).normalize());
                } else {
                    enemy_projectiles.push(Projectile::new(Circle::newv(self.pos.center(), (PLAYER_RADIUS/6) as f32), (player.center() - self.pos.center()).normalize() * 4));
                }
            }
        }
    }
}
