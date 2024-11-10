use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_spawn)
            .add_systems(Update, update_camera);
    }
}
