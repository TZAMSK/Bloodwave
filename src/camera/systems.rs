use bevy::prelude::*;

use crate::player::components::Player;

const CAM_LERP_FACTOR: f32 = 8.0;

pub fn camera_spawn(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

pub fn update_camera(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    camera.translation = camera
        .translation
        .lerp(direction, time.delta_seconds() * CAM_LERP_FACTOR)
}
