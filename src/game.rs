use super::*;

pub struct LoadResults {
    pub player_image: Image,
    pub crosshair: Image,
    pub gun: Image,
    pub wood: Image,
    pub shadow: Image,
    pub wall: Image,
    pub bat: Image,
    pub medic: Image,
    pub spider: Image,
    pub angry_spider: Image,
    pub gear: Image,
    pub fire: Sound,
    pub death: Sound
}

pub struct GameScreen {
    pub player_down: Option<Circle>,
    pub player_pos: Circle,
    pub cord_pos: Circle,
    pub enemies: Vec<Enemy>,
    pub enemy_buffer: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub player_image: Image,
    pub crosshair: Image,
    pub gun: Image,
    pub wood: Image,
    pub shadow: Image,
    pub wall: Image,
    pub fire: Sound,
    pub bat_up: Image,
    pub bat_down: Image,
    pub medic: Image,
    pub spider: Image,
    pub angry_spider: Image,
    pub gear: Image,
    pub death: Sound,
    pub bat_frame: u32,
    pub wall_scroll: f32,
    pub shoot_cooldown: i32,
    pub combat_roll: i32,
    pub adrenaline: f32,
    pub elevation: i32,
    pub cord_health: f32,
    pub gear_spin: f32
}

impl GameScreen {
    pub fn new(load: LoadResults) -> GameScreen {
        GameScreen {
            player_down: Option::None,
            player_pos: Circle::new(100, 100, PLAYER_RADIUS),
            cord_pos: Circle::new(960/2, 540/2, 48),
            enemies: Vec::new(),
            enemy_buffer: Vec::new(),
            projectiles: Vec::new(),
            player_image: load.player_image,
            crosshair: load.crosshair,
            gun: load.gun,
            wood: load.wood,
            shadow: load.shadow,
            wall: load.wall,
            medic: load.medic,
            bat_up: load.bat.subimage(Rectangle::new(0, 0, 16, 16)),
            bat_down: load.bat.subimage(Rectangle::new(16, 0, 16, 16)),
            death: load.death,
            bat_frame: 0,
            spider: load.spider,
            angry_spider: load.angry_spider,
            gear: load.gear,
            fire: load.fire,
            wall_scroll: 0.0,
            shoot_cooldown: 0,
            combat_roll: 0,
            adrenaline: 0.0,
            elevation: 0,
            cord_health: CORD_HEALTH,
            gear_spin: 0.0,
        }
    }
}

const MAX_ADRENALINE: f32 = 100.0; //Internal 100% adrenaline
const COMBAT_ROLL: i32 = 15; //Duration in ticks of combat roll
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

impl Screen for GameScreen {
    fn update(&mut self, window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        let keyboard = window.keyboard();
        if self.combat_roll > 0 {
            self.combat_roll -= 1;
        }
        if keyboard[Key::Space].is_down() {
            self.combat_roll = COMBAT_ROLL;
        }
        self.player_pos.x += if keyboard[Key::D].is_down() { PLAYER_SPEED * if self.combat_roll > 0 { COMBAT_ROLL_SPEED_FACTOR } else { 1.0 } } else { 0.0 };
        self.player_pos.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED * if self.combat_roll > 0 { COMBAT_ROLL_SPEED_FACTOR } else { 1.0 } } else { 0.0 };
        self.player_pos.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED * if self.combat_roll > 0 { COMBAT_ROLL_SPEED_FACTOR } else { 1.0 } } else { 0.0 };
        self.player_pos.y += if keyboard[Key::S].is_down() { PLAYER_SPEED * if self.combat_roll > 0 { COMBAT_ROLL_SPEED_FACTOR } else { 1.0 } } else { 0.0 };
        if keyboard[Key::LShift].is_down() && !keyboard[Key::D].is_down() && !keyboard[Key::W].is_down() && !keyboard[Key::A].is_down() && !keyboard[Key::S].is_down() &&
                self.player_pos.overlaps_circ(self.cord_pos) {
            self.cord_health += MAX_REPAIR_SPEED - (MAX_REPAIR_SPEED - MIN_REPAIR_SPEED) * self.adrenaline / MAX_ADRENALINE;
        }
        if window.mouse().left().is_down() && self.shoot_cooldown <= 0 && self.player_down == Option::None {
            let mut rng = rand::thread_rng();
            self.fire.play();
            self.projectiles.push(Projectile::new(Circle::newv(self.player_pos.center(), (PLAYER_RADIUS/4) as f32),
                    Transform::rotate(rng.gen_range(-(MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE - MIN_GUN_SPREAD,
                    (MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE + MIN_GUN_SPREAD))
                    * (window.mouse().pos() - self.player_pos.center()).normalize() * PLAYER_BULLET_SPEED, ProjectileType::PlayerBullet));
            self.shoot_cooldown = MAX_SHOOT_COOLDOWN - ((MAX_SHOOT_COOLDOWN - MIN_SHOOT_COOLDOWN) as f32 * self.adrenaline / MAX_ADRENALINE) as i32;
        }
        if self.shoot_cooldown > 0 {
            self.shoot_cooldown -= 1;
        }
        if window.mouse().right().is_down() && self.player_down == Option::None {
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
            e.update(self.player_pos, self.cord_pos, &mut self.cord_health, &mut self.projectiles, &mut self.enemy_buffer);
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
                if p.pos.overlaps_circ(e.pos) {
                    e.remove = true;
                    p.remove = true;
                    self.adrenaline += ADRENALINE_GAIN;
                }
            }
        }
        if self.player_down == Option::None {
            for p in self.projectiles.iter_mut() {
                if p.proj_type != ProjectileType::EnemyBullet {
                    continue;
                }
                if p.pos.overlaps_circ(self.player_pos) && self.combat_roll <= 0 {
                    let mut rng = rand::thread_rng();
                    p.remove = true;
                    self.player_down = Option::Some(self.player_pos);
                    self.player_pos = Circle::new(rng.gen_range(0.0, 960.0), rng.gen_range(0.0, 540.0), self.player_pos.radius);
                }
            }
        }
        if let Some(player_down) = self.player_down {
            if player_down.overlaps_circ(self.player_pos) {
                self.player_pos = player_down;
                self.player_down = Option::None;
            }
        }
        let death = self.death.clone();
        clean_list(&mut self.enemies, || death.play());
        clean_list(&mut self.projectiles, ||());
        //Enemies by elevation function:
        //1.5 * sin(e / 200) + 1.2root(e / 350 + 2)
        self.elevation += 1;
        while (self.enemies.len() as f32) < 1.5 * (self.elevation as f32 / 200.0).sin() + (self.elevation as f32 / 350.0 + 2.0).powf(1.0 / 1.2) {
            self.enemies.push(Enemy::gen_new());
        }
        self.adrenaline -= ADRENALINE_DRAIN;
        if self.adrenaline < 0.0 {
            self.adrenaline = 0.0;
        } else if self.adrenaline > MAX_ADRENALINE {
            self.adrenaline = MAX_ADRENALINE;
        }
        self.wall_scroll = (self.wall_scroll + 0.1) % 64.0;
        self.bat_frame = (self.bat_frame + 1) % 60;
        self.gear_spin = (self.gear_spin + 0.25) % 360.0;
        None
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        let double = Transform::scale(Vector::new(2, 2));
        for x in 0..30 {
            for y in 0..2 {
                canvas.draw_image_trans(&self.wall, Vector::new(x as f32 * 64.0 - 32.0, y as f32 * 64.0 - 32.0 + self.wall_scroll), Color::white(), double);
            }
        }
        let left_gear_rotation = Transform::rotate(-self.gear_spin);
        canvas.draw_image_trans(&self.gear, Vector::new(26, 64), Color::white(), left_gear_rotation * double);
        let right_gear_rotation = Transform::rotate(self.gear_spin);
        canvas.draw_image_trans(&self.gear, Vector::new(960 - 26, 64), Color::white(), right_gear_rotation * double);
        for x in 0..30 {
            for y in 2..17 {
                canvas.draw_image_trans(&self.wood, Vector::new(x as f32 * 64.0 - 32.0, y as f32 * 64.0 - 32.0), Color::white(), double);
            }
        }
        let left_gear_rotation = Transform::rotate(-self.gear_spin);
        canvas.draw_image_trans(&self.gear, Vector::new(26, 550), Color::white(), left_gear_rotation * double);
        let right_gear_rotation = Transform::rotate(self.gear_spin);
        canvas.draw_image_trans(&self.gear, Vector::new(960 - 26, 550), Color::white(), right_gear_rotation * double);
        //Draw the player
        match self.player_down {
            Option::Some(player_down) => {
                canvas.draw_image_trans(&self.shadow, player_down.center() + Vector::y() * 24, Color::white(), double);
                canvas.draw_image_trans(&self.player_image, player_down.center(), Color::white(),
                                        Transform::rotate(self.combat_roll as f32 / 15.0 * 360.0)
                                        * double);
                canvas.draw_image_trans(&self.shadow, self.player_pos.center() + Vector::y() * 24, Color::white(), double);
                canvas.draw_image_trans(&self.medic, self.player_pos.center(), Color::white(), double);
            },
            Option::None => {
                canvas.draw_image_trans(&self.shadow, self.player_pos.center() + Vector::y() * 24, Color::white(), double);
                canvas.draw_image_trans(&self.player_image, self.player_pos.center(), Color::white(),
                                        Transform::rotate(self.combat_roll as f32 / 15.0 * 360.0)
                                        * double);
                //Draw the player's weapon
                let point = window.mouse().pos() - self.player_pos.center();
                let rotation = point.angle();
                let scale = Vector::new(1.0, point.x.signum());
                canvas.draw_image_trans(&self.gun, self.player_pos.center(), Color::white(),
                                        Transform::translate(Vector::new(0, 10))
                                        * Transform::rotate(rotation)
                                        * double
                                        * Transform::scale(scale)
                                        * Transform::translate(Vector::new(12, 0)));
            }
        }
        for e in self.enemies.iter() {
            let image = if self.bat_frame > 30 { &self.bat_up } else { &self.bat_down };
            let shadow_offset = match e.enemy_type {
                EnemyType::Bat => 24,
                _ => 4
            };
            canvas.draw_image_trans(&self.shadow, e.pos.center() + Vector::y() * shadow_offset, Color::white(), double);
            match e.enemy_type {
                EnemyType::MamaSpider(_, _) => canvas.draw_circle(e.pos, Color::purple()),
                EnemyType::AngrySpider(_) => canvas.draw_image_trans(&self.angry_spider, e.pos.center(), Color::white(), double),
                EnemyType::Spider(_) => canvas.draw_image_trans(&self.spider, e.pos.center(), Color::white(), double),
                EnemyType::Bat => canvas.draw_image_trans(image, e.pos.center(), Color::white(), double),
                EnemyType::Gunner(_) => canvas.draw_circle(e.pos, Color::red())
            }
        }
        for p in self.projectiles.iter() {
            canvas.draw_circle(p.pos, Color::yellow());
        }
        canvas.draw_circle(self.cord_pos, Color::blue());
        canvas.draw_rect(Rectangle::new(960.0/2.0-200.0, 10.0, 400.0 * self.cord_health / CORD_HEALTH, 20.0), Color::green());
        canvas.draw_rect(Rectangle::new(960.0/2.0-100.0, 35.0, 200.0 * self.adrenaline / MAX_ADRENALINE, 15.0), Color::blue());
        canvas.draw_image_trans(&self.crosshair, window.mouse().pos(), Color::white(), double);
        canvas.present(window);
    }

}

fn clean_list<T: Killable, F: Fn()>(list: &mut Vec<T>, on_death: F) {
    let mut i = 0;
    while i < list.len() {
        if list[i].is_dead() {
            list.remove(i);
            on_death();
        } else {
            i += 1;
        }
    }
}
