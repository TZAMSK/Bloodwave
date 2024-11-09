mod player;
mod systems;

use bevy::prelude::*;
use player::PlayerPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}
