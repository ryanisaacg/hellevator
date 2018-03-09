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
                enemies.push(Enemy::new(Circle::new(50, 100, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(960-50, 100, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(50, 540-50, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(960-50, 540-50, 50), EnemyType::GearLeg));
            }
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
