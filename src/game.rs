use super::*;
use rand::Rng;

pub struct LoadResults {
    pub player_image: Image,
    pub crosshair: Image,
    pub gun: Image,
    pub wood: Image,
    pub shadow: Image,
    pub wall: Image,
    pub bat: Image,
    pub fire: Sound,
    pub death: Sound
}

pub struct GameScreen {
    pub player_pos: Circle,
    pub cord_pos: Circle,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub enemy_projectiles: Vec<Projectile>,
    pub player_image: Image,
    pub crosshair: Image,
    pub gun: Image,
    pub wood: Image,
    pub shadow: Image,
    pub wall: Image,
    pub fire: Sound,
    pub bat_up: Image,
    pub bat_down: Image,
    pub death: Sound,
    pub bat_frame: u32,
    pub wall_scroll: f32,
    pub shoot_cooldown: i32,
    pub combat_roll: i32,
    pub adrenaline: f32,
    pub cord_health: f32
}

impl GameScreen {
    pub fn new(load: LoadResults) -> GameScreen {
        GameScreen {
            player_pos: Circle::newi(100, 100, PLAYER_RADIUS),
            cord_pos: Circle::newi(960/2, 540/2, 48),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            enemy_projectiles: Vec::new(),
            player_image: load.player_image,
            crosshair: load.crosshair,
            gun: load.gun,
            wood: load.wood,
            shadow: load.shadow,
            wall: load.wall,
            bat_up: load.bat.subimage(Rectangle::newi(0, 0, 16, 16)),
            bat_down: load.bat.subimage(Rectangle::newi(16, 0, 16, 16)),
            death: load.death,
            bat_frame: 0,
            fire: load.fire,
            wall_scroll: 0.0,
            shoot_cooldown: 0,
            combat_roll: 0,
            adrenaline: 0.0,
            cord_health: CORD_HEALTH
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
const PLAYER_BULLET_SPEED: f32 = 5.0; //Velocity of player bullets
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
        if window.mouse().left().is_down() && self.shoot_cooldown <= 0 {
            let mut rng = rand::thread_rng();
            self.fire.play();
            self.projectiles.push(Projectile::new(Circle::newv(self.player_pos.center(), (PLAYER_RADIUS/8) as f32),
                    Transform::rotate(rng.gen_range(-(MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE - MIN_GUN_SPREAD,
                    (MAX_GUN_SPREAD - MIN_GUN_SPREAD) * self.adrenaline / MAX_ADRENALINE + MIN_GUN_SPREAD))
                    * (window.mouse().pos() - self.player_pos.center()).normalize() * PLAYER_BULLET_SPEED));
            self.shoot_cooldown = MAX_SHOOT_COOLDOWN - ((MAX_SHOOT_COOLDOWN - MIN_SHOOT_COOLDOWN) as f32 * self.adrenaline / MAX_ADRENALINE) as i32;
        }
        if self.shoot_cooldown > 0 {
            self.shoot_cooldown -= 1;
        }
        if window.mouse().right().is_down() {
            for p in self.projectiles.iter_mut() {
                let mut rng = rand::thread_rng();
                p.vel = (window.mouse().pos() + Vector::new(rng.gen_range(-(REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE - REDIRECT_MIN_RANGE,
                        (REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE + REDIRECT_MIN_RANGE),
                        rng.gen_range(-(REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE - REDIRECT_MIN_RANGE,
                        (REDIRECT_MAX_RANGE - REDIRECT_MIN_RANGE) * self.adrenaline / MAX_ADRENALINE + REDIRECT_MIN_RANGE)) - p.pos.center()).normalize() * PLAYER_BULLET_SPEED;
            }
        }
        for e in self.enemies.iter_mut() {
            e.update(self.player_pos, self.cord_pos, &mut self.cord_health, &mut self.enemy_projectiles);
        }
        for p in self.projectiles.iter_mut() {
            p.update();
        }
        for p in self.enemy_projectiles.iter_mut() {
            p.update();
        }
        for p in self.projectiles.iter_mut() {
            for e in self.enemies.iter_mut() {
                if p.pos.overlaps_circ(e.pos) {
                    e.remove = true;
                    p.remove = true;
                    self.adrenaline += ADRENALINE_GAIN;
                }
            }
        }
        for p in self.enemy_projectiles.iter_mut() {
            if p.pos.overlaps_circ(self.player_pos) && self.combat_roll <= 0 {
                p.remove = true;
                //TODO Player dies here
            }
        }
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].remove {
                self.enemies.remove(i);
                self.death.play();
            } else {
                i += 1;
            }
        }
        i = 0;
        while i < self.projectiles.len() {
            if self.projectiles[i].remove {
                self.projectiles.remove(i);
            } else {
                i += 1;
            }
        }
        i = 0;
        while i < self.enemy_projectiles.len() {
            if self.enemy_projectiles[i].remove {
                self.enemy_projectiles.remove(i);
            } else {
                i += 1;
            }
        }
        while self.enemies.len() < 4 {
            let mut rng = rand::thread_rng();
            let x: i32 = if rng.gen() { rng.gen_range(0, 960) } else { 0 };
            let y: i32 = if x == 0 { rng.gen_range(0, 540) } else { 0 };
            self.enemies.push(Enemy::new(Circle::newi(x, y, PLAYER_RADIUS/2), if rng.gen() { EnemyType::Bat } else { EnemyType::Gunner(0) }));
        }
        self.adrenaline -= ADRENALINE_DRAIN;
        if self.adrenaline < 0.0 {
            self.adrenaline = 0.0;
        } else if self.adrenaline > MAX_ADRENALINE {
            self.adrenaline = MAX_ADRENALINE;
        }
        self.wall_scroll = (self.wall_scroll + 0.1) % 64.0;
        self.bat_frame = (self.bat_frame + 1) % 60;
        None
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        let double = Transform::scale(Vector::newi(2, 2));
        for x in 0..30 {
            for y in 0..17 {
                let image = if y < 2 { &self.wall } else { &self.wood };
                let offset = if y < 2 { self.wall_scroll } else { 0.0 };
                canvas.draw_image_trans(image, Vector::new(x as f32 * 64.0 - 32.0, y as f32 * 64.0 - 32.0 + offset), Color::white(), double);
            }
        }
        //Draw the player
        canvas.draw_image_trans(&self.shadow, self.player_pos.center() + Vector::y() * 24, Color::white(), double);
        canvas.draw_image_trans(&self.player_image, self.player_pos.center(), Color::white(),
                                Transform::rotate(self.combat_roll as f32 / 15.0 * 360.0)
                                * double);
        //Draw the player's weapon
        let point = window.mouse().pos() - self.player_pos.center();
        let rotation = point.angle();
        let scale = Vector::new(1.0, point.x.signum());
        canvas.draw_image_trans(&self.gun, self.player_pos.center(), Color::white(),
                                Transform::translate(Vector::newi(0, 10))
                                * Transform::rotate(rotation)
                                * double
                                * Transform::scale(scale)
                                * Transform::translate(Vector::newi(12, 0)));
        for e in self.enemies.iter() {
            let image = if self.bat_frame > 30 { &self.bat_up } else { &self.bat_down };
            canvas.draw_image_trans(&self.shadow, e.pos.center() + Vector::y() * 24, Color::white(), double);
            match e.enemy_type {
                EnemyType::Bat => canvas.draw_image_trans(image, e.pos.center(), Color::white(), double),
                EnemyType::Gunner(_) => canvas.draw_circle(e.pos, Color::red())
            }
        }
        for p in self.projectiles.iter() {
            canvas.draw_circle(p.pos, Color::yellow());
        }
        for p in self.enemy_projectiles.iter() {
            canvas.draw_circle(p.pos, Color::yellow());
        }
        canvas.draw_circle(self.cord_pos, Color::blue());
        canvas.draw_rect(Rectangle::new(960.0/2.0-200.0, 10.0, 400.0 * self.cord_health / CORD_HEALTH, 20.0), Color::green());
        canvas.draw_rect(Rectangle::new(960.0/2.0-100.0, 35.0, 200.0 * self.adrenaline / MAX_ADRENALINE, 15.0), Color::blue());
        canvas.draw_image_trans(&self.crosshair, window.mouse().pos(), Color::white(), double);
        canvas.present(window);
    }

}
