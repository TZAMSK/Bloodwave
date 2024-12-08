use crate::player::components::Player;
use rand::Rng;

use super::components::*;
use super::types::melee::MeleeEnemy;
use super::types::EnemyTypes;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const NUMBER_ENEMIES: u32 = 19;

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
                texture: asset_server.load("sprites/enemies/spider.png"),
                ..default()
            },
            Enemy::new(EnemyTypes::Melee(MeleeEnemy::Spider)),
        ));

        i += 1;
    }
}

pub fn enemy_movement(
    player_query: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&Enemy, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some(player) = player_query.iter().next() {
        for (enemy, mut transform_enemy) in enemy_query.iter_mut() {
            let mut enemy_position = transform_enemy.translation;
            let player_position = player.translation;
            let direction = player_position.distance(enemy_position);

            let distance = player_position - enemy_position;

            let angle = distance.y.atan2(distance.x);
            if direction <= enemy.aggro_range {
                transform_enemy.rotation = Quat::from_rotation_z(angle);
                enemy_position += distance.normalize() * enemy.speed * time.delta_seconds();
                transform_enemy.translation = enemy_position
            }
        }
    }
}

pub fn correct_collision_enemies(mut enemy_query: Query<(&Enemy, &mut Transform)>) {
    let mut enemy_data: Vec<(Transform, f32)> = Vec::new();

    for (enemy, transform_enemy) in enemy_query.iter() {
        enemy_data.push((transform_enemy.clone(), enemy.size));
    }

    for i in 0..enemy_data.len() {
        for j in (i + 1)..enemy_data.len() {
            let (transform_a, size_a) = &enemy_data[i];
            let (transform_b, size_b) = &enemy_data[j];

            let position_a = transform_a.translation;
            let position_b = transform_b.translation;

            let current_distance = position_a.distance(position_b);
            let distance_needed = size_a / 2.0 + size_b / 2.0;

            if current_distance < distance_needed {
                let correction = (distance_needed - current_distance) / 2.0;
                let direction = (position_b - position_a).normalize();

                enemy_data[i].0.translation -= direction * correction;
                enemy_data[j].0.translation += direction * correction;
            }
        }
    }

    for (i, (_, mut transform)) in enemy_query.iter_mut().enumerate() {
        transform.translation = enemy_data[i].0.translation;
    }
}
