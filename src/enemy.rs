use super::*;

#[derive(Copy, Clone)]
pub enum EnemyType {
    BoomSpider(i32),
    WebSpider(i32),
    MamaSpider(i32, Vector),
    AngrySpider(i32),
    Spider(i32, u32),
    Bat
}

pub struct Enemy {
    pub pos: Circle,
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub remove: bool
}

impl Enemy {
    pub fn new(pos: Circle, enemy_type: EnemyType) -> Enemy {
        let h = match enemy_type {
            EnemyType::BoomSpider(_) => 99999.0,
            EnemyType::WebSpider(_) => 12.0,
            EnemyType::MamaSpider(_, _) => 25.0,
            EnemyType::AngrySpider(_) => 9.0,
            EnemyType::Spider(_, _) => 10.0,
            EnemyType::Bat => 1.0
        };
        Enemy { pos, enemy_type, health: h, max_health: h, remove: false }
    }

    pub fn gen_new() -> Enemy {
        let mut rng = rand::thread_rng();
        let mut pos = Circle::new(0.0, 0.0, 9999999.0);
        while pos.overlaps_rect(Rectangle::new(960.0/2.0 - 200.0, 540.0/2.0 - 100.0, 400.0, 200.0)) {
            pos = Circle::new(rng.gen_range(0, 960), rng.gen_range(0, 540), PLAYER_RADIUS/2);
        }
        let types = [/*EnemyType::Bat*/ EnemyType::Spider(0, 0), EnemyType::AngrySpider(0), EnemyType::MamaSpider(0, Vector::zero()),
                EnemyType::WebSpider(0), EnemyType::BoomSpider(0)];
        if let Some(enemy_type) = rng.choose(&types) {
            Enemy::new(pos, *enemy_type)
        } else {
            Enemy::new(pos, EnemyType::Bat)
        }
    }

    pub fn update(&mut self, player: Circle, cord_pos: Circle, cord_health: &mut f32, projectiles: &mut Vec<Projectile>, enemy_buffer: &mut Vec<Enemy>) {
        match self.enemy_type {
            EnemyType::BoomSpider(ref mut jump_cycle) => {
                *jump_cycle = (*jump_cycle + 1) % 45;
                if *jump_cycle > 29 {
                    let mut rng = rand::thread_rng();
                    self.pos = self.pos.translate(Transform::rotate(rng.gen_range(-30.0, 30.0)) * (cord_pos.center() - self.pos.center()).normalize() * (45 - *jump_cycle) / 2);
                }
                if self.pos.overlaps_circ(cord_pos) {
                    self.health -= 1.0;
                }
                if self.health < self.max_health {
                    self.remove = true;
                    if (self.pos.center() - player.center()).len2() < 150.0*150.0 {
                        //TODO kill player from here
                    }
                    if (self.pos.center() - cord_pos.center()).len2() < 150.0*150.0 {
                        *cord_health -= 50.0;
                    }
                }
            },
            EnemyType::WebSpider(ref mut jump_cycle) => {
                *jump_cycle = (*jump_cycle + 1) % 90;
                if *jump_cycle > 74 {
                    let mut rng = rand::thread_rng();
                    self.pos = self.pos.translate(Transform::rotate(rng.gen_range(-30.0, 30.0)) * (player.center() - self.pos.center()).normalize() * (90 - *jump_cycle) / 2);
                }
                if *jump_cycle >= 89 && (self.pos.center() - player.center()).len2() < 300.0*300.0 {
                    projectiles.push(Projectile::new(Circle::newv(self.pos.center(), (PLAYER_RADIUS/6) as f32), (player.center() - self.pos.center()).normalize() * 4, ProjectileType::Web(0)));
                }
            },
            EnemyType::MamaSpider(ref mut jump_cycle, ref mut jump_direction) => {
                let mut rng = rand::thread_rng();
                *jump_cycle = (*jump_cycle + 1) % 150;
                if *jump_cycle == 134 {
                    *jump_direction = (rng.gen::<Vector>() - Vector::one() * 0.5).normalize();
                } else if *jump_cycle > 134 {
                    self.pos = self.pos.translate(*jump_direction * (150 - *jump_cycle) / 2);
                }
                if *jump_cycle >= 149 && rng.gen_range(0.0, 1.0) < 0.3 {
                    let types = [EnemyType::Spider(0, 0), EnemyType::AngrySpider(0)];
                    if let Some(enemy_type) = rng.choose(&types) {
                        enemy_buffer.push(Enemy::new(self.pos, *enemy_type));
                    }
                }
            },
            EnemyType::AngrySpider(ref mut jump_cycle) => {
                *jump_cycle = (*jump_cycle + 1) % 90;
                if *jump_cycle > 74 {
                    let mut rng = rand::thread_rng();
                    self.pos = self.pos.translate(Transform::rotate(rng.gen_range(-30.0, 30.0)) * (player.center() - self.pos.center()).normalize() * (90 - *jump_cycle) / 2);
                }
                if *jump_cycle >= 89 && (self.pos.center() - player.center()).len2() < 200.0*200.0 {
                    projectiles.push(Projectile::new(Circle::newv(self.pos.center(), (PLAYER_RADIUS/6) as f32), (player.center() - self.pos.center()).normalize() * 4, ProjectileType::EnemyBullet));
                }
            },
            EnemyType::Spider(ref mut jump_cycle, ref mut frame) => {
                *jump_cycle = (*jump_cycle + 1) % 60;
                if *jump_cycle > 44 {
                    let mut rng = rand::thread_rng();
                    self.pos = self.pos.translate(Transform::rotate(rng.gen_range(-30.0, 30.0)) * (cord_pos.center() - self.pos.center()).normalize() * (60 - *jump_cycle) / 2);
                    *frame = (*frame + 1) % 30;
                }
                if self.pos.overlaps_circ(cord_pos) {
                    self.remove = true;
                    *cord_health -= 10.0;
                }
            },

            // LEGACY EXAMPLES
            EnemyType::Bat => {
                self.pos = self.pos.translate((cord_pos.center() - self.pos.center()).normalize() * 2);

                if self.pos.overlaps_circ(cord_pos) {
                    self.remove = true;
                    *cord_health -= 10.0;
                }
            }
        }
    }
}

impl Killable for Enemy {
    fn is_dead(&self) -> bool {
        self.remove
    }
}
