mod camera;
mod player;
mod systems;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Update, exit_game)
        .run();
}
