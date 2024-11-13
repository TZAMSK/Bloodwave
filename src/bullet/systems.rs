use super::components::*;
use crate::player::components::Player;
use bevy::prelude::*;

const TIME_BETWEEN_SHOTS: f32 = 0.12;
const BULLET_SPEED: f32 = 2000.0;

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
                Bullet {},
            ));
        }
    }
}

pub fn bullet_movement(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut bullet in bullet_query.iter_mut() {
        let direction = bullet.rotation.mul_vec3(Vec3::X);

        bullet.translation += direction.normalize() * BULLET_SPEED * time.delta_seconds();
    }
}
