use super::components::*;
use crate::enemy::components::{Enemy, XP};
use crate::player::components::Player;
use bevy::prelude::*;
use rand::Rng;

const TIME_BETWEEN_SHOTS: f32 = 0.12;

#[derive(Resource)]
pub struct ShootTimer(Timer);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(ShootTimer(Timer::from_seconds(
        TIME_BETWEEN_SHOTS,
        TimerMode::Repeating,
    )));
}

pub fn bullet_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    mut shoot_timer: ResMut<ShootTimer>,
) {
    shoot_timer.0.tick(time.delta());

    if shoot_timer.0.finished() && mouse_input.pressed(MouseButton::Left) {
        if let Some(player) = player_query.iter().next() {
            let player_pos = player.translation;
            let player_rotation = player.rotation;

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(player_pos.x, player_pos.y, player_pos.z),
                        rotation: player_rotation,
                        ..default()
                    },
                    texture: asset_server.load("sprites/bullet.png"),
                    ..default()
                },
                Bullet { ..default() },
            ));
        }
    }
}

pub fn bullet_movement(
    mut bullet_query: Query<(&Bullet, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (bullet, mut bullet_transform) in bullet_query.iter_mut() {
        let direction = bullet_transform.rotation.mul_vec3(Vec3::X);

        bullet_transform.translation += direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

pub fn bullet_hit_enemy(
    mut commands: Commands,
    bullet_query: Query<(&Bullet, Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(&Enemy, Entity, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    for (bullet, bullet_entity, bullet_transform) in bullet_query.iter() {
        for (enemy, enemy_entity, enemy_transform) in enemy_query.iter() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);
            let bullet_radius = bullet.size / 2.0;
            let enemy_radius = enemy.size / 2.0;
            if distance < bullet_radius + enemy_radius {
                let spawn_y: f32 = rng.gen_range(-40.0..40.0);
                let spawn_x: f32 = rng.gen_range(-40.0..40.0);
                let chance: f32 = rng.gen_range(0.0..1.0);

                commands.entity(bullet_entity).despawn();
                commands.entity(enemy_entity).despawn();

                if chance <= enemy.xp.drop_chance {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_xyz(
                                enemy_transform.translation.x + spawn_x,
                                enemy_transform.translation.y + spawn_y,
                                1.0,
                            ),
                            texture: asset_server.load("sprites/xp.png"),
                            ..default()
                        },
                        XP { ..default() },
                    ));
                }
            }
        }
    }
}
