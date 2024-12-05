use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub size: f32,
    pub speed: f32,
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            size: 20.0,
            speed: 2000.0,
        }
    }
}
