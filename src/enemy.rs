use super::*;

#[derive(Copy, Clone)]
pub enum EnemyType {
    Bat,
    Gunner(i32)
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

    pub fn gen_new() -> Enemy {
        let mut rng = rand::thread_rng();
        let mut pos = Circle::new(0.0, 0.0, 9999999.0);
        while pos.overlaps_rect(Rectangle::new(960.0/2.0 - 200.0, 540.0/2.0 - 100.0, 400.0, 200.0)) {
            pos = Circle::newi(rng.gen_range(0, 960), rng.gen_range(0, 540), PLAYER_RADIUS/2);
        }
        let types: Vec<EnemyType> = vec![EnemyType::Bat, EnemyType::Gunner(0)];
        if let Some(enemy_type) = rng.choose(&types) {
            Enemy { pos, enemy_type: *enemy_type, remove: false }
        } else {
            Enemy { pos, enemy_type: EnemyType::Bat, remove: false }
        }
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
            EnemyType::Gunner(ref mut shoot_cooldown) => {
                if *shoot_cooldown > 0 {
                    *shoot_cooldown -= 1;
                }
                if (self.pos.center() - player.center()).len2() > 500.0*500.0 {
                    self.pos = self.pos.translate((player.center() - self.pos.center()).normalize());
                } else {
                    if *shoot_cooldown <= 0 {
                        enemy_projectiles.push(Projectile::new(Circle::newv(self.pos.center(), (PLAYER_RADIUS/6) as f32), (player.center() - self.pos.center()).normalize() * 4));
                        *shoot_cooldown = 15;
                    }
                }
            }
        }
    }
}
