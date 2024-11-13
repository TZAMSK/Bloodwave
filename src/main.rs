mod bullet;
mod camera;
mod enemy;
mod player;
mod systems;
mod world;

use bevy::prelude::*;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::*;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(EnemyPlugin)
        .add_systems(Update, exit_game)
        .run();
}
