use bevy::prelude::*;

mod systems_ui_stats;
mod systems_ui_xp;

use systems_ui_stats::*;
use systems_ui_xp::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, base_stat_bar_spawn)
            .add_systems(Startup, xp_bar_spawn);
    }
}
