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
                enemies.retain(|enemy| match enemy.enemy_type { EnemyType::SpiderLeg(_, _) => false, _ => true });
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(0, false)));
                enemies.push(Enemy::new(Circle::new(100, 100, 50), EnemyType::SpiderLeg(10, false)));
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
                let gear_count = enemies.iter().filter(|e| e.enemy_type == EnemyType::GearLeg).count();
                let leg_count = enemies.iter().filter(|e| match e.enemy_type { EnemyType::SpiderLeg(_, _) => true, _ => false }).count();
                if 6 - gear_count != leg_count {
                    enemies.push(Enemy::new(Circle::new(480, 260, 50), EnemyType::SpiderLeg(0, false)));
                }
                let gears_present = gear_count != 0;
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
