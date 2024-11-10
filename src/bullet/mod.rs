use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, bullet_spawn)
            .add_systems(Update, bullet_movement);
    }
}
