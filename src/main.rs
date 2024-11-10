mod camera;
mod player;
mod systems;
mod world;

use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use systems::*;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Update, exit_game)
        .run();
}
