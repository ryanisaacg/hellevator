use super::*;

pub struct GameScreen {
    pub player_pos: Circle,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub player_image: Image,
    pub crosshair: Image,
    pub shoot_cooldown: i32
}

impl Screen for GameScreen {
    fn update(&mut self, window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        let keyboard = window.keyboard();
        self.player_pos.x += if keyboard[Key::D].is_down() { PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::W].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.x += if keyboard[Key::A].is_down() { -PLAYER_SPEED } else { 0.0 };
        self.player_pos.y += if keyboard[Key::S].is_down() { PLAYER_SPEED } else { 0.0 };
        if window.mouse().left().is_down() && self.shoot_cooldown <= 0 {
            self.projectiles.push(Projectile::new(Circle::newv(self.player_pos.center(), (PLAYER_RADIUS/8) as f32), (window.mouse().pos() - self.player_pos.center()).normalize() * 5));
            self.shoot_cooldown = 10;
        }
        if self.shoot_cooldown > 0 {
            self.shoot_cooldown -= 1;
        }
        if window.mouse().right().is_down() {
            for p in self.projectiles.iter_mut() {
                p.vel = (window.mouse().pos() - p.pos.center()).normalize() * 5;
            }
        }
        for e in self.enemies.iter_mut() {
            e.update(self.player_pos);
        }
        for p in self.projectiles.iter_mut() {
            p.update();
        }
        for p in self.projectiles.iter_mut() {
            for e in self.enemies.iter_mut() {
                if p.pos.overlaps_circ(e.pos) {
                    e.remove = true;
                    p.remove = true;
                }
            }
        }
        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].remove {
                self.enemies.remove(i);
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
        while self.enemies.len() < 4 {
            self.enemies.push(Enemy::new(Circle::newi(0, 0, PLAYER_RADIUS/2)));
        }
        None
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        canvas.draw_image_trans(&self.player_image, self.player_pos.center(), Color::white(), Transform::scale(Vector::newi(2, 2)));
        for e in self.enemies.iter() {
            canvas.draw_circle(e.pos, Color::red());
        }
        for p in self.projectiles.iter() {
            canvas.draw_circle(p.pos, Color::yellow());
        }
        canvas.draw_image_trans(&self.crosshair, window.mouse().pos(), Color::white(), Transform::scale(Vector::newi(2, 2)));
        canvas.present(window);
    }

}
