use bevy::prelude::*;

use crate::player::components::Player;

use super::components::MyCamera;

pub fn camera_spawn(mut commands: Commands) {
    commands.spawn((Camera2dBundle { ..default() }, MyCamera { ..default() }));
}

pub fn update_camera(
    mut camera: Query<(&MyCamera, &mut Transform), (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok((camera, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    let Ok(player) = player.get_single() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera_transform.translation.z);

    camera_transform.translation = camera_transform
        .translation
        .lerp(direction, time.delta_seconds() * camera.lerp_factor)
}
