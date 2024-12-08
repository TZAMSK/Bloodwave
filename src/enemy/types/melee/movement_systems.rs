use crate::enemy::components::Enemy;
use crate::player::components::Player;
use bevy::prelude::*;

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
