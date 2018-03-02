use super::*;
use game::GAME_AREA;

#[derive(Copy, Clone, PartialEq)]
pub enum EnemyType {
    SpiderLeg(i32),
    BufferSpider(AttackState),
    Egg(i32),
    BoomSpider(i32),
    WebSpider(i32),
    MamaSpider(i32),
    AngrySpider(i32),
    Spider(i32, u32),
    Bat
}

#[derive(Copy, Clone, PartialEq)]
pub enum AttackState {
    Punch(i32),
    Web(i32),
    Summon(i32)
}

pub enum UpdateResult {
    HitPlayer,
    None
}

pub struct Enemy {
    pub pos: Circle,
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub invulnerable: bool,
    pub remove: bool,
    pub velocity: Vector
}

impl Enemy {
    pub fn new(pos: Circle, enemy_type: EnemyType) -> Enemy {
        let h = match enemy_type {
            EnemyType::SpiderLeg(_) => 100.0,
            EnemyType::BufferSpider(_) => 250.0,
            EnemyType::Egg(_) => 7.0,
            EnemyType::BoomSpider(_) => 99999.0,
            EnemyType::WebSpider(_) => 12.0,
            EnemyType::MamaSpider(_) => 25.0,
            EnemyType::AngrySpider(_) => 9.0,
            EnemyType::Spider(_, _) => 10.0,
            EnemyType::Bat => 1.0
        };
        let invulnerable = if let EnemyType::SpiderLeg(_) = enemy_type { true } else { false };
        Enemy { pos, enemy_type, health: h, max_health: h, invulnerable, remove: false, velocity: Vector::zero() }
    }

    pub fn gen_new() -> Enemy {
        let mut rng = rand::thread_rng();
        let types = [/*EnemyType::Bat*/ EnemyType::Spider(0, 0), EnemyType::AngrySpider(0), EnemyType::MamaSpider(0),
                EnemyType::WebSpider(0), EnemyType::BoomSpider(0), EnemyType::BufferSpider(AttackState::Punch(0))];
        if let Some(enemy_type) = rng.choose(&types) {
            let mut pos = Circle::new(0.0, 0.0, 9999999.0);
            while pos.overlaps_rect(Rectangle::new(960.0/2.0 - 200.0, 540.0/2.0 - 100.0, 400.0, 200.0)) {
                pos = Circle::new(rng.gen_range(0, 960), rng.gen_range(0, 540), PLAYER_RADIUS/2 * if let EnemyType::BufferSpider(_) = *enemy_type { 2 } else { 1 });
            }
            Enemy::new(pos, *enemy_type)
        } else {
            Enemy::new(Circle::new(0.0, 0.0, (PLAYER_RADIUS/2) as f32), EnemyType::Bat)
        }
    }

    pub fn update(&mut self, player: Circle, cord_pos: Circle, cord_health: &mut f32, projectiles: &mut Vec<Projectile>, enemy_buffer: &mut Vec<Enemy>) -> UpdateResult {
        let mut result = UpdateResult::None;
        match self.enemy_type {
            EnemyType::SpiderLeg(ref mut cycle) => {
                *cycle += 1;
                if *cycle < 150 {
                    self.pos = self.pos.translate(((player.center() - self.pos.center()) + Transform::rotate(*cycle * 6) * Vector::x() * 20) / (16 + *cycle/10));
                }
                if *cycle >= 210 {
                    //TODO Attack
                    *cycle = 0;
                }
            },
            EnemyType::BufferSpider(ref mut attack_state) => {
                let mut rng = rand::thread_rng();
                let mut new_attack = false;
                match *attack_state {
                    AttackState::Punch(ref mut cycle) => {
                        *cycle += 1;
                        if *cycle % 60 == 29 {
                            self.velocity = Transform::rotate(rng.gen_range(-30.0, 30.0)) * (player.center() - self.pos.center()).normalize();
                        }
                        if *cycle % 60 > 29 {
                            self.pos = self.pos.translate(self.velocity * (65 - *cycle % 60) / 2);
                        }
                        if *cycle > 200 {
                            new_attack = true;
                        }
                        if self.pos.overlaps_circ(player) {
                            result = UpdateResult::HitPlayer;
                        }
                    },
                    AttackState::Web(ref mut cycle) => {
                        *cycle += 1;
                        if *cycle % 60 == 29 {
                            self.velocity = Transform::rotate(rng.gen_range(-30.0, 30.0)) * (player.center() - self.pos.center()).normalize();
                        }
                        if *cycle % 60 > 29 {
                            self.pos = self.pos.translate(self.velocity * (65 - *cycle % 60) / 2);
                        }
                        if *cycle > 60 {
                            new_attack = true;
                            for t in 0..5 {
                                projectiles.push(Projectile::new(Circle::newv(self.pos.center(), (PLAYER_RADIUS/6) as f32),
                                        Transform::rotate(t as f32 * 360.0/5.0 + 90.0) * (player.center() - self.pos.center()).normalize() * 4, ProjectileType::Web(60)));
                            }
                        }
                    },
                    AttackState::Summon(ref mut cycle) => {
                        *cycle += 1;
                        if *cycle == 90 {
                            for i in 0..4 {
                                enemy_buffer.push(Enemy::new(Circle::newv(self.pos.center() + Transform::rotate(i as f32*90.0 + 45.0) * Vector::x() * 16, PLAYER_RADIUS/3), EnemyType::Egg(0)));
                            }
                        }
                        if *cycle > 120 {
                            new_attack = true;
                        }
                    }
                }
                if new_attack {
                    let attacks = [AttackState::Punch(0), AttackState::Web(0), AttackState::Summon(0)];
                    if let Some(attack) = rng.choose(&attacks) {
                        *attack_state = *attack;
                    }
                }
            },
            EnemyType::Egg(ref mut timer) => {
                *timer += 1;
                if *timer > 420 {
                    self.remove = true;
                    enemy_buffer.push(Enemy::new(self.pos, EnemyType::Spider(0, 0)));
                }
            },
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
                        result = UpdateResult::HitPlayer;
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
            EnemyType::MamaSpider(ref mut jump_cycle) => {
                let mut rng = rand::thread_rng();
                *jump_cycle = (*jump_cycle + 1) % 150;
                if *jump_cycle == 134 {
                    self.velocity = (rng.gen::<Vector>() - Vector::one() * 0.5).normalize();
                } else if *jump_cycle > 134 {
                    self.pos = self.pos.translate(self.velocity * (150 - *jump_cycle) / 2);
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
        self.pos = self.pos.translate(self.velocity).constrain(GAME_AREA);
        result
    }
}

impl Killable for Enemy {
    fn is_dead(&self) -> bool {
        self.remove
    }
}
