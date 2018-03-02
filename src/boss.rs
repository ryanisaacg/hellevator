use super::*;

#[derive(PartialEq)]
pub enum Boss {
    Spider,
    None
}

impl Boss {
    pub fn setup(&mut self, enemies: &mut Vec<Enemy>) {
        match self {
            Spider => {
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(0)));
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(10)));
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(20)));
            },
            _ => {}
        }
    }

    pub fn update(&mut self) {
        match self {
            Spider => {

            },
            _ => {}
        }
    }
}
