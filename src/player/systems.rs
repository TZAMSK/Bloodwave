use super::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::FRAC_PI_2;

pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
            texture: asset_server.load("sprites/survivor.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_rotate(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(mut transform) = player_query.get_single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            let player_position = transform.translation;

            let mouse_target = Vec3::new(cursor_position.x, cursor_position.y, 0.0) - FRAC_PI_2;

            let direction = mouse_target - player_position;
            let angle = direction.y.atan2(direction.x);
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
