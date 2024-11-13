use crate::player::components::Player;
use rand::Rng;

use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const ENEMY_SPEED: f32 = 500.0;
const DISTANCE_ENEMY_PLAYER: f32 = 400.0;
const NUMBER_ENEMIES: u32 = 4;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let window = window_query.get_single().unwrap();

    let mut i = 0;
    while i <= NUMBER_ENEMIES {
        let spawn_y: f32 = rng.gen_range(0.0..540.0);
        let spawn_x: f32 = rng.gen_range(0.0..960.0);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    window.width() / 2.0 - spawn_x,
                    window.height() / 2.0 - spawn_y,
                    1.0,
                ),
                texture: asset_server.load("sprites/hitman.png"),
                ..default()
            },
            Enemy {},
        ));

        i += 1;
    }
}

pub fn enemy_movement(
    player_query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
) {
    if let Some(player) = player_query.iter().next() {
        for (mut transform_enemy, enemy) in enemy_query.iter_mut() {
            let mut enemy_position = transform_enemy.translation;
            let player_position = player.translation;
            let direction = player_position.distance(enemy_position);

            let distance = player_position - enemy_position;

            let angle = distance.y.atan2(distance.x);
            if direction <= DISTANCE_ENEMY_PLAYER {
                transform_enemy.rotation = Quat::from_rotation_z(angle);
                enemy_position += distance.normalize() * ENEMY_SPEED * time.delta_seconds();
            }
        }
    }
}
