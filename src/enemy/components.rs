use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub aggro_range: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 200.0,
            aggro_range: 400.0,
        }
    }
}
