use super::components::XPBarFill;
use crate::player::components::Player;
use bevy::prelude::*;

pub fn xp_bar_spawn(mut commands: Commands) {
    commands
        // Main Node for stats
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(32.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgba(0.9, 0.9, 0.9, 0.02)),
                    ..default()
                })
                .with_children(|child| {
                    child
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(0.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::srgba(0.35, 0.51, 0.448, 0.2)),
                            ..default()
                        })
                        .insert(XPBarFill);
                });
        });
}

pub fn update_xp_bar(
    mut player_query: Query<&mut Player>,
    mut bar_query: Query<&mut Style, With<XPBarFill>>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if let Ok(mut style) = bar_query.get_single_mut() {
            let progress = player.xp_progress() * 100.0;
            style.width = Val::Percent(progress);
        }
    }
}
