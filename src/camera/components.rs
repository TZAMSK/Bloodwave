use bevy::prelude::*;

#[derive(Component)]
pub struct MyCamera {
    pub lerp_factor: f32,
}

impl Default for MyCamera {
    fn default() -> Self {
        Self { lerp_factor: 8.0 }
    }
}
