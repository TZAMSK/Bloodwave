use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub health: f32,
    pub shield: f32,
    pub stamina: f32,
    pub loot_magnet: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 500.0,
            health: 100.0,
            shield: 10.0,
            stamina: 10.0,
            loot_magnet: 1.0,
        }
    }
}
