use super::*;

pub struct GameScreen {
    player_down: Option<Circle>,
    player_pos: Circle,
    cord_pos: Circle,
    boss: Boss,
    enemies: Vec<Enemy>,
    enemy_buffer: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    shoot_cooldown: i32,
    combat_roll: i32,
    roll_cooldown: i32,
    adrenaline: f32,
    web_timer: i32,
    elevation: i32,
    cord_health: f32,
    particles: Vec<Particle>,
    player_velocity: Vector,
    assets: Assets
}

const MAX_ADRENALINE: f32 = 100.0; //Internal 100% adrenaline
const COMBAT_ROLL_DURATION: i32 = 15; //Duration in ticks of combat roll
const COMBAT_ROLL_COOLDOWN: i32 = 30; //Ticks between being able to activeate combat rolls
const COMBAT_ROLL_SPEED_FACTOR: f32 = 1.75; //Factor by which speed is multiplied during combat roll
const MIN_REPAIR_SPEED: f32 = 0.02; //Repair speed at maximum adrenaline
const MAX_REPAIR_SPEED: f32 = 0.05; //Repair speed at minimum adrenaline
const MIN_GUN_SPREAD: f32 = 5.0; //Angle of deviation of firing at minimum adrenaline
const MAX_GUN_SPREAD: f32 = 20.0; //Angle of deviation of firing at maximum adrenaline
const PLAYER_BULLET_SPEED: f32 = 15.0; //Velocity of player bullets
const MAX_SHOOT_COOLDOWN: i32 = 10; //Duration in ticks between shots at minimum adrenaline
const MIN_SHOOT_COOLDOWN: i32 = 6; //Duration in ticks between shots at maximum adrenaline
const REDIRECT_MIN_RANGE: f32 = 10.0; //Square radius of inaccuracy of right click at minimum adrenaline
const REDIRECT_MAX_RANGE: f32 = 30.0; //Square radius of inaccuracy of right click at maximum adrenaline
const ADRENALINE_GAIN: f32 = 2.0; //Amount of adrenaline gained for each hit
const ADRENALINE_DRAIN: f32 = 0.005; //Amount of adrenaline passively lost per tick
const WEB_SLOWDOWN: f32 = 0.2; //Factor the web effect slows you by
pub const GAME_AREA: Rectangle = Rectangle { x: 0.0, y: 64.0, width: 960.0, height: 476.0 }; //The size of the elevator floor
const PLAYER_DEATH_PROJECTILES: u32 = 40; //The amount of projectiles spawned when the player dies

impl GameScreen {
    pub fn new(assets: Assets) -> GameScreen {
        GameScreen {
            player_down: Option::None,
            player_pos: Circle::new(100, 100, PLAYER_RADIUS),
            cord_pos: Circle::new(960/2, 540/2, 48),
            boss: Boss::None,
            enemies: Vec::new(),
            enemy_buffer: Vec::new(),
            projectiles: Vec::new(),
            shoot_cooldown: 0,
            roll_cooldown: 0,
            combat_roll: 0,
            adrenaline: 0.0,
            web_timer: 0,
            elevation: 0,
            cord_health: CORD_HEALTH,
            particles: Vec::new(),
            player_velocity: Vector::zero(),
            assets
        }
    }

    fn player_hit(player_down: &mut Option<Circle>, player_pos: &mut Circle, projectiles: &mut Vec<Projectile>) {
        if *player_down == None {
            let mut rng = rand::thread_rng();
            *player_down = Option::Some(*player_pos);
            for i in 0..PLAYER_DEATH_PROJECTILES {
                let angle = 360.0 * i as f32 / PLAYER_DEATH_PROJECTILES as f32;
                projectiles.push(Projectile::new(Circle::newv(player_pos.center(), (PLAYER_RADIUS/4) as f32), Vector::from_angle(angle) * 5, ProjectileType::PlayerBullet));
            }
            *player_pos = Circle::new(rng.gen_range(0.0, 960.0), rng.gen_range(0.0, 540.0), player_pos.radius);
        }
    }

    pub fn update(&mut self, window: &mut Window) {
        let keyboard = window.keyboard();
        let mut projectile_buffer = Vec::new();
        if self.combat_roll > 0 {
            self.combat_roll -= 1;
        }
        if self.roll_cooldown > 0 {
            self.roll_cooldown -= 1;
        }
        if keyboard[Key::Space] == ButtonState::Pressed && self.roll_cooldown <= 0 {
            self.combat_roll = COMBAT_ROLL_DURATION;
            self.roll_cooldown = COMBAT_ROLL_COOLDOWN;
        }
        if self.web_timer > 0 {
            self.web_timer -= 1;
        }

        let player_move = if self.combat_roll <= 0 {
            let mut player_move = Vector::zero();
            player_move.x += if keyboard[Key::D].is_down() { PLAYER_SPEED } else { 0.0 };
            player_move.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED } else { 0.0 };
            player_move.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED } else { 0.0 };
            player_move.y += if keyboard[Key::S].is_down() { PLAYER_SPEED } else { 0.0 };
            player_move *= if self.web_timer > 0 { WEB_SLOWDOWN } else { 1.0 };
            self.player_velocity = player_move * COMBAT_ROLL_SPEED_FACTOR;
            player_move
        } else {
            self.player_velocity
        };
        self.player_pos = self.player_pos.translate(player_move).constrain(GAME_AREA);

        if keyboard[Key::LShift].is_down() && !keyboard[Key::D].is_down() && !keyboard[Key::W].is_down() && !keyboard[Key::A].is_down() && !keyboard[Key::S].is_down() &&
                self.player_pos.overlaps_circ(self.cord_pos) {
            self.cord_health += MAX_REPAIR_SPEED - (MAX_REPAIR_SPEED - MIN_REPAIR_SPEED) * self.adrenaline / MAX_ADRENALINE;
        }
        if window.mouse()[MouseButton::Left].is_down() && self.shoot_cooldown <= 0 && self.player_down == Option::None {
            let mut rng = rand::thread_rng();
            self.assets.fire.play();
            self.projectiles.push(Projectile::new(Circle::newv(self.player_pos.center(), (PLAYER_RADIUS/4) as f32),
                    Transform::rotate(rng.gen_range(-(MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE - MIN_GUN_SPREAD,
                    (MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE + MIN_GUN_SPREAD))
                    * (window.mouse().pos() - self.player_pos.center()).normalize() * PLAYER_BULLET_SPEED, ProjectileType::PlayerBullet));
            self.shoot_cooldown = MAX_SHOOT_COOLDOWN - ((MAX_SHOOT_COOLDOWN - MIN_SHOOT_COOLDOWN) as f32 * self.adrenaline / MAX_ADRENALINE) as i32;
        }
        if self.shoot_cooldown > 0 {
            self.shoot_cooldown -= 1;
        }
        if window.mouse()[MouseButton::Right].is_down() && self.player_down == Option::None {
            for p in self.projectiles.iter_mut() {
                if p.proj_type != ProjectileType::PlayerBullet {
                    continue;
                }
                let mut rng = rand::thread_rng();
                p.vel = (window.mouse().pos() + Vector::new(rng.gen_range(-(REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE - REDIRECT_MIN_RANGE,
                        (REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE + REDIRECT_MIN_RANGE),
                        rng.gen_range(-(REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE - REDIRECT_MIN_RANGE,
                        (REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE + REDIRECT_MIN_RANGE)) - p.pos.center()).normalize() * PLAYER_BULLET_SPEED;
            }
        }
        for e in self.enemies.iter_mut() {
            let result = e.update(self.player_pos, self.cord_pos, &mut self.cord_health, &mut self.projectiles, &mut self.enemy_buffer);
            match result {
                UpdateResult::HitPlayer => GameScreen::player_hit(&mut self.player_down, &mut self.player_pos, &mut projectile_buffer),
                UpdateResult::None => ()
            }
        }
        self.enemies.append(&mut self.enemy_buffer);
        for p in self.projectiles.iter_mut() {
            p.update();
        }
        for p in self.projectiles.iter_mut() {
            if p.proj_type != ProjectileType::PlayerBullet {
                continue;
            }
            for e in self.enemies.iter_mut() {
                if p.pos.overlaps_circ(e.pos) && !e.invulnerable {
                    e.health -= 2.0 + 2.0 * self.adrenaline / MAX_ADRENALINE;
                    e.remove = e.health <= 0.0;
                    p.remove = true;
                    self.adrenaline += ADRENALINE_GAIN;
                }
            }
        }
        if let Some(player_down) = self.player_down {
            if player_down.overlaps_circ(self.player_pos) {
                self.player_pos = player_down;
                self.player_down = Option::None;
                let mut rng = rand::thread_rng();
                for _ in 0..10 {
                    self.particles.push(Particle {
                        image: self.assets.plus.clone(),
                        pos: self.player_pos.center(),
                        velocity: (rng.gen::<Vector>() - Vector::new(0.5, 0.5)) * 6,
                        rotation: 0.0,
                        rotational_velocity: 0.0,
                        lifetime: 20
                    })
                }
            }
        } else {
            for p in self.projectiles.iter_mut() {
                if p.proj_type == ProjectileType::PlayerBullet {
                    continue;
                }
                if p.pos.overlaps_circ(self.player_pos) && self.combat_roll <= 0 {
                    p.remove = true;
                    if p.proj_type == ProjectileType::EnemyBullet {
                        GameScreen::player_hit(&mut self.player_down, &mut self.player_pos, &mut projectile_buffer);
                    } else if let ProjectileType::Web(_) = p.proj_type {
                        self.web_timer = 120;
                    }
                }
            }
        }
        let death = self.assets.death.clone();
        let mut particles = Vec::with_capacity(0);
        let death_particle = self.assets.enemy_death_particle.clone();
        clean_list(&mut self.enemies, |enemy| {
            death.play();
            let amount_particles = match enemy.enemy_type {
                EnemyType::GearLeg => 60,
                EnemyType::SpiderLeg(_) => 0,
                EnemyType::BufferSpider(_) => 42,
                EnemyType::Egg(_) => 2,
                EnemyType::BoomSpider(_) => 4,
                EnemyType::WebSpider(_) => 6,
                EnemyType::MamaSpider(_) => 12,
                EnemyType::AngrySpider(_) => 5,
                EnemyType::Spider(_, _) => 3,
                EnemyType::Bat => 1
            };
            for _ in 0..amount_particles {
                let mut rng = rand::thread_rng();
                particles.push(Particle {
                            image: death_particle.clone(),
                            pos: enemy.pos.center(),
                            velocity: (rng.gen::<Vector>() - Vector::new(0.5, 0.5)) * 6,
                            rotation: 0.0,
                            rotational_velocity: 0.0,
                            lifetime: 10
                        })
            }
        });
        self.particles.extend(particles);
        clean_list(&mut self.projectiles, |_|());
        if !self.enemies.iter().any(|e| e.enemy_type == EnemyType::GearLeg) {
            self.elevation += 1;
        }
        //Enemies by elevation function:
        //1.5 * sin(e / 200) + 1.2root(e / 350 + 2)
        // while (self.enemies.len() as f32) < 1.5 * (self.elevation as f32 / 200.0).sin() + (self.elevation as f32 / 350.0 + 2.0).powf(1.0 / 1.2) {
        //     self.enemies.push(Enemy::gen_new());
        // }
        if self.boss == Boss::None {
            if self.elevation == 200 {
                self.boss = Boss::Spider;
                self.boss.setup(&mut self.enemies);
            }
        } else {
            self.boss.update();
        }
        self.adrenaline -= ADRENALINE_DRAIN;
        if self.adrenaline < 0.0 {
            self.adrenaline = 0.0;
        } else if self.adrenaline > MAX_ADRENALINE {
            self.adrenaline = MAX_ADRENALINE;
        }
        //Do the particle update
        for particle in self.particles.iter_mut() {
            particle.pos += particle.velocity;
            particle.rotation += particle.rotational_velocity;
            particle.lifetime -= 1;
        }
        clean_list(&mut self.particles, |_|());
        self.projectiles.extend(projectile_buffer);
    }

    fn wall_scroll(&self) -> f32 {
        (self.elevation as f32 * 0.1) % 64.0
    }

    fn gear_spin(&self) -> f32 {
        (self.elevation as f32 * 0.25) % 360.0
    }

    pub fn draw(&mut self, window: &mut Window) {
        use std::iter::once;
        let double = Transform::scale(Vector::new(2, 2));
        let mut draw_items = Vec::new();
        let wall_z = -1000;
        let back_gear_z = -900;
        let front_gear_z = 1000;
        let shadow_z = 10;
        let projectile_z = 1500;
        let center_z = 2000;
        let ui_z = 3000;
        //Draw particles
        draw_items.extend(self.particles.iter().map(|x|
            DrawCall::image(&x.image, x.pos).with_transform(Transform::rotate(x.rotation))));
        //Draw walls
        draw_items.extend(iproduct!(0..30, 0..2).map(|(x, y)|
                DrawCall::image(&self.assets.wall, Vector::new(x as f32 * 64.0 - 32.0, y as f32 * 64.0 - 32.0 + self.wall_scroll()))
                    .with_transform(double)
                    .with_z(wall_z)));
        let left_gear_rotation = Transform::rotate(-self.gear_spin()) * double;
        let right_gear_rotation = Transform::rotate(self.gear_spin()) * double;
        //Draw the wire
        draw_items.extend((0..5).map(|y| DrawCall::image(&self.assets.wire, Vector::new(480.0, y as f32 * 64.0 - 32.0 + self.wall_scroll()))
                    .with_transform(double)
                    .with_z(y * 64 + 50)));
        //Draw gears
        draw_items.extend_from_slice(&[
            DrawCall::image(&self.assets.gear, Vector::new(26, 64)).with_transform(left_gear_rotation).with_z(back_gear_z),
            DrawCall::image(&self.assets.gear, Vector::new(960 - 26, 64)).with_transform(right_gear_rotation).with_z(back_gear_z),
            DrawCall::image(&self.assets.gear, Vector::new(26, 550)).with_transform(left_gear_rotation).with_z(front_gear_z),
            DrawCall::image(&self.assets.gear, Vector::new(960 - 26, 550)).with_transform(right_gear_rotation).with_z(front_gear_z)
        ]);
        draw_items.extend(iproduct!(0..30, 2..17).map(|(x, y)|
                DrawCall::image(&self.assets.wood, Vector::new(x as f32 * 64.0 - 32.0, y as f32 * 64.0 - 32.0))
                    .with_transform(double)));
        //Draw the player
        match self.player_down {
            Option::Some(player_down) => draw_items.extend_from_slice(&[
                DrawCall::image(&self.assets.shadow, player_down.center() + Vector::y() * 24).with_transform(double).with_z(shadow_z),
                DrawCall::image(&self.assets.player_image, player_down.center()).with_transform(double).with_z(player_down.center().y),
                DrawCall::image(&self.assets.shadow, self.player_pos.center() + Vector::y() * 24).with_transform(double).with_z(shadow_z),
                DrawCall::image(&self.assets.medic, self.player_pos.center()).with_transform(double).with_z(self.player_pos.center().y)
            ]),
            Option::None => {
                let point = window.mouse().pos() - self.player_pos.center();
                let rotation = point.angle();
                let scale = Vector::new(1.0, point.x.signum());
                let gun_transform = Transform::translate(Vector::new(0, 10))
                                        * Transform::rotate(rotation)
                                        * double
                                        * Transform::scale(scale)
                                        * Transform::translate(Vector::new(12, 0));
                draw_items.extend_from_slice(&[
                    DrawCall::image(&self.assets.shadow, self.player_pos.center() + Vector::y() * 24).with_transform(double).with_z(shadow_z),
                    DrawCall::image(&self.assets.player_image, self.player_pos.center())
                        .with_color(if self.web_timer > 0 { Color { r: 1.5, g: 1.5, b: 1.5, a: 1.0 } } else { Color::white() })
                        .with_transform(Transform::rotate(self.combat_roll as f32 / 15.0 * 360.0) * double).with_z(self.player_pos.center().y),
                    DrawCall::image(&self.assets.gun, self.player_pos.center()).with_transform(gun_transform).with_z(self.player_pos.center().y)
                ])
            }
        }
        // Draw enemies
        draw_items.extend(self.enemies.iter().flat_map(|e| {
            let (shadow_offset, shadow_size) = match e.enemy_type {
                EnemyType::Bat => (24, 1.0),
                EnemyType::MamaSpider(_) => (8, 1.0),
                EnemyType::Egg(_) => (8, 0.9),
                EnemyType::SpiderLeg(_) => (2, 2.0),
                EnemyType::BufferSpider(_) => (12, 2.0),
                _ => (4, 1.0)
            };
            once(DrawCall::image(&self.assets.shadow, e.pos.center() + Vector::y() * shadow_offset).with_transform(double * Transform::scale(Vector::one() * shadow_size)).with_z(shadow_z))
                .chain(once(match e.enemy_type {
                    EnemyType::GearLeg => DrawCall::circle(e.pos).with_color(Color::orange()),
                    EnemyType::SpiderLeg(_) => /*TODO Eventually draw leg when stabs*/DrawCall::rectangle(Rectangle::new(0, 0, 0, 0)),
                    EnemyType::BoomSpider(_) => DrawCall::image(&self.assets.explode_spider, e.pos.center()).with_transform(double),
                    EnemyType::WebSpider(_) => DrawCall::image(&self.assets.web_spider, e.pos.center()).with_transform(double),
                    EnemyType::BufferSpider(_) => DrawCall::image(&self.assets.buffer_spider, e.pos.center()).with_transform(double),
                    EnemyType::Egg(_) => DrawCall::image(&self.assets.egg, e.pos.center()).with_transform(double),
                    EnemyType::MamaSpider(_) => DrawCall::image(&self.assets.mama_spider, e.pos.center()).with_transform(double),
                    EnemyType::AngrySpider(_) => DrawCall::image(&self.assets.angry_spider, e.pos.center()).with_transform(double),
                    EnemyType::Spider(jump, frame) => DrawCall::image(if jump > 44 { &self.assets.spider_skitter[(frame / 15) as usize] } else { &self.assets.spider }, e.pos.center()).with_transform(double),
                    EnemyType::Bat => DrawCall::image(&self.assets.bat_up, e.pos.center()).with_transform(double)
                }.with_z(e.pos.y)))
        }));
        // Draw projectiles
        draw_items.extend(self.projectiles.iter().map(|projectile| match projectile.proj_type {
            ProjectileType::PlayerBullet => DrawCall::circle(projectile.pos).with_color(Color::yellow()).with_z(projectile_z),
            ProjectileType::EnemyBullet => DrawCall::circle(projectile.pos).with_color(Color::red()).with_z(projectile_z),
            ProjectileType::Web(ticks) if ticks <= 90 => DrawCall::circle(projectile.pos).with_color(Color::white()).with_z(projectile_z),
            ProjectileType::Web(_) => DrawCall::image(&self.assets.spiderweb, projectile.pos.center()).with_transform(double).with_z(projectile_z)
        }));
        // Draw UI / misc
        draw_items.extend_from_slice(&[
            DrawCall::circle(self.cord_pos).with_color(Color::black()).with_z(center_z),
            DrawCall::rectangle(Rectangle::new(960.0/2.0-200.0, 10.0, 400.0 * self.cord_health / CORD_HEALTH, 20.0)).with_color(Color::green()).with_z(ui_z),
            DrawCall::rectangle(Rectangle::new(960.0/2.0-100.0, 35.0, 200.0 * self.adrenaline / MAX_ADRENALINE, 15.0)).with_color(Color::blue()).with_z(ui_z),
            DrawCall::image(&self.assets.crosshair, window.mouse().pos()).with_transform(double).with_z(ui_z)
        ]);
        window.draw(draw_items.iter());
    }

}

fn clean_list<T, F>(list: &mut Vec<T>, mut on_death: F) where T: Killable, F: FnMut(T) {
    let mut i = 0;
    while i < list.len() {
        if list[i].is_dead() {
            on_death(list.remove(i));
        } else {
            i += 1;
        }
    }
}
