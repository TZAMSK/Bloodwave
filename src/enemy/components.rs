use super::xp::XP;
use crate::enemy::types::boss::BossEnemy;
use crate::enemy::types::melee::MeleeEnemy;
use crate::enemy::types::range::RangeEnemy;
use crate::enemy::types::support::SupportEnemy;
use crate::enemy::types::EnemyTypes;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyTypes,
    pub health: f32,
    pub speed: f32,
    pub aggro_range: f32,
    pub xp: XP,
    pub level: u32,
    pub size: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyTypes) -> Self {
        let (health, speed, aggro_range, size, level, xp) = match &enemy_type {
            EnemyTypes::Melee(melee) => melee.stats(),
            EnemyTypes::Range(range) => range.stats(),
            EnemyTypes::Support(support) => support.stats(),
            EnemyTypes::Boss(boss) => boss.stats(),
        };

        Self {
            enemy_type,
            health,
            speed,
            aggro_range,
            xp,
            level,
            size,
        }
    }
}
