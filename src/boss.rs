use super::*;

#[derive(PartialEq)]
pub enum Boss {
    Spider,
    None
}

impl Boss {
    pub fn setup(&mut self, enemies: &mut Vec<Enemy>) {
        match self {
            &mut Boss::Spider => {
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(0, false)));
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(10, false)));
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(20, false)));
                enemies.push(Enemy::new(Circle::new(50, 100, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(960-50, 100, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(50, 540-50, 50), EnemyType::GearLeg));
                enemies.push(Enemy::new(Circle::new(960-50, 540-50, 50), EnemyType::GearLeg));
            },
            _ => {}
        }
    }

    pub fn update(&mut self, enemies: &mut Vec<Enemy>) {
        match self {
            &mut Boss::Spider => {
                let gears_present = enemies.iter().any(|e| e.enemy_type == EnemyType::GearLeg);
                for enemy in enemies.iter_mut() {
                    if let EnemyType::SpiderLeg(_, ref mut is_angry) = enemy.enemy_type {
                        *is_angry = !gears_present
                    }
                }
            },
            _ => {}
        }
    }
}
