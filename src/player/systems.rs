use super::components::*;
use crate::enemy::xp::XP;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const XP_PICK_UP_SPEED: f32 = 500.0;

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
        Player { ..default() },
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player, mut player_transform)) = player_query.get_single_mut() {
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

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * player.speed * time.delta_seconds();
    }
}

pub fn player_rotate(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    camera_query: Query<(&GlobalTransform, &Camera), With<Camera>>,
) {
    let window = window_query.get_single().unwrap();

    let (camera_transform, camera) = camera_query.get_single().unwrap();

    if let Ok(mut transform) = player_query.get_single_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            if let Some(world_position) =
                camera.viewport_to_world(camera_transform, cursor_position)
            {
                let player_position = transform.translation;

                let direction = world_position.origin - player_position;

                let angle = direction.y.atan2(direction.x);

                transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

pub fn pick_up_xp(
    mut commands: Commands,
    mut xp_query: Query<(&XP, Entity, &mut Transform), (With<XP>, Without<Player>)>,
    player_query: Query<(&mut Player, &Transform), (With<Player>, Without<XP>)>,
    time: Res<Time>,
) {
    if let Ok((player, player_transform)) = player_query.get_single() {
        let player_position = player_transform.translation;

        for (xp, xp_entity, mut xp_transform) in xp_query.iter_mut() {
            let mut xp_position = xp_transform.translation;
            let distance = player_position - xp_position;
            let distance_length = distance.length();
            let xp_radius = xp.size / 2.0;
            let player_radius = player.size / 2.0;

            if distance_length < xp_radius + player_radius {
                let angle_to_player = distance.y.atan2(distance.x);
                let rotation_offset = (time.elapsed_seconds() * XP_PICK_UP_SPEED).sin();
                let curved_angle = angle_to_player + rotation_offset;
                let movement_direction = Vec3::new(curved_angle.cos(), curved_angle.sin(), 0.0);

                xp_position += movement_direction * XP_PICK_UP_SPEED * time.delta_seconds();
                xp_transform.rotation = Quat::from_rotation_z(curved_angle);
                xp_transform.translation = xp_position;
            }

            let destroy_proximity = player_position.distance(xp_position);

            if destroy_proximity < 1.0 {
                commands.entity(xp_entity).despawn();
            }
        }
    }
}
