use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinedSystemSet;

pub enum PlayerSystemSet {
    Movement,
    Confined,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                player_movement
                    .before(confine_player_movement)
                    .in_set(MovementSystemSet),
            )
            .add_systems(
                Update,
                player_rotate
                    .before(confine_player_movement)
                    .in_set(MovementSystemSet),
            )
            .add_systems(Update, confine_player_movement.in_set(ConfinedSystemSet));
    }
}
