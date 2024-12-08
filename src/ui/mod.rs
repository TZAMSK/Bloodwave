use bevy::prelude::*;

mod components;
mod systems_ui_stats;
mod systems_ui_xp;

use systems_ui_stats::*;
use systems_ui_xp::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, base_stat_bar_spawn)
            .add_systems(Startup, xp_bar_spawn)
            .add_systems(Update, update_xp_bar)
            .add_systems(Update, update_level_indicator)
            .add_systems(Update, update_health_bar)
            .add_systems(Update, update_shield_bar)
            .add_systems(Update, update_stamina_bar);
    }
}
