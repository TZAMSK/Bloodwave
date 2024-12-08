use bevy::prelude::*;

mod systems;

use systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, base_stat_bar_spawn);
    }
}
