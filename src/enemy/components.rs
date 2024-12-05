use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub aggro_range: f32,
    pub xp: XP,
    pub level: u32,
}

#[derive(Component)]
pub struct XP {
    pub xp: f32,
    pub drop_chance: f32,
    pub distance_pick_up: f32,
    pub speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 200.0,
            aggro_range: 400.0,
            xp: XP::default(),
            level: 1,
        }
    }
}

impl Default for XP {
    fn default() -> Self {
        Self {
            xp: 5.0,
            drop_chance: 0.4,
            distance_pick_up: 50.0,
            speed: 300.0,
        }
    }
}
