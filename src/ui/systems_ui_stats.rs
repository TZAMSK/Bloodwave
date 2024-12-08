use super::components::{HealthBarFill, LevelIndicator, ShieldBarFill, StaminaBarFill};
use crate::player::components::Player;
use bevy::prelude::*;

pub fn base_stat_bar_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Main Node for stats
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(30.0),
                height: Val::Px(180.0),
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                left: Val::Px(20.0),
                top: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Left: Level
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(180.0),
                        height: Val::Px(180.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|left| {
                    // Image Frame
                    left.spawn(ImageBundle {
                        image: UiImage {
                            texture: asset_server.load("sprites/hud/stats/frame.png"),
                            ..default()
                        },
                        style: Style {
                            width: Val::Px(180.0),
                            height: Val::Px(180.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|frame| {
                        frame
                            .spawn(TextBundle {
                                text: Text::from_section(
                                    "?",
                                    TextStyle {
                                        font: asset_server.load("fonts/Pixel.ttf"),
                                        font_size: 70.0,
                                        color: Color::srgb(
                                            192.0 / 255.0,
                                            143.0 / 255.0,
                                            88.0 / 255.0,
                                        ),
                                    },
                                ),
                                style: Style { ..default() },
                                ..default()
                            })
                            .insert(LevelIndicator);
                    });
                });

            // Right: Stats
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|right| {
                    // Health
                    right
                        .spawn(ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("sprites/hud/stats/health.png"),
                                ..default()
                            },
                            style: Style {
                                width: Val::Px(360.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(40.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgba(
                                        0.298, 0.06, 0.05, 0.4,
                                    )),
                                    ..default()
                                })
                                .with_children(|child| {
                                    child
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(40.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgba(
                                                0.415, 0.09, 0.074, 0.5,
                                            )),
                                            ..default()
                                        })
                                        .insert(HealthBarFill);
                                });
                        });
                    // Shield
                    right
                        .spawn(ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("sprites/hud/stats/shield.png"),
                                ..default()
                            },
                            style: Style {
                                width: Val::Px(360.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(40.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgba(
                                        0.062, 0.1333, 0.372, 0.4,
                                    )),
                                    ..default()
                                })
                                .with_children(|child| {
                                    child
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(40.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgba(
                                                0.109, 0.235, 0.572, 0.5,
                                            )),
                                            ..default()
                                        })
                                        .insert(ShieldBarFill);
                                });
                        });
                    // Stamina
                    right
                        .spawn(ImageBundle {
                            image: UiImage {
                                texture: asset_server.load("sprites/hud/stats/stamina.png"),
                                ..default()
                            },
                            style: Style {
                                width: Val::Px(360.0),
                                height: Val::Px(40.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(40.0),
                                        ..default()
                                    },
                                    background_color: BackgroundColor(Color::srgba(
                                        0.113, 0.4078, 0.07, 0.4,
                                    )),
                                    ..default()
                                })
                                .with_children(|child| {
                                    child
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(40.0),
                                                ..default()
                                            },
                                            background_color: BackgroundColor(Color::srgba(
                                                0.184, 0.517, 0.094, 0.5,
                                            )),
                                            ..default()
                                        })
                                        .insert(StaminaBarFill);
                                });
                        });
                });
        });
}

pub fn update_level_indicator(
    player_query: Query<&Player>,
    mut level_indicator_query: Query<&mut Text, With<LevelIndicator>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut level_indicator) = level_indicator_query.get_single_mut() {
            let level = player.level.to_string();
            level_indicator.sections[0].value = level
        }
    }
}

pub fn update_health_bar(
    player_query: Query<&Player>,
    mut health_bar_query: Query<&mut Style, With<LevelIndicator>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut health_bar) = health_bar_query.get_single_mut() {
            let current_health = player.core_stats.health.current_health;
            let max_health = player.core_stats.health.max_health;
            let health = (current_health / max_health) * 100.0;
            health_bar.width = Val::Percent(health)
        }
    }
}

pub fn update_shield_bar(
    player_query: Query<&Player>,
    mut shield_bar_query: Query<&mut Style, With<LevelIndicator>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut shield_bar) = shield_bar_query.get_single_mut() {
            let current_shield = player.core_stats.shield.current_shield;
            let max_shield = player.core_stats.shield.max_shield;
            let shield = (current_shield / max_shield) * 100.0;
            shield_bar.width = Val::Percent(shield)
        }
    }
}

pub fn update_stamina_bar(
    player_query: Query<&Player>,
    mut stamina_bar_query: Query<&mut Style, With<LevelIndicator>>,
) {
    if let Ok(player) = player_query.get_single() {
        if let Ok(mut stamina_bar) = stamina_bar_query.get_single_mut() {
            let current_stamina = player.core_stats.stamina.current_stamina;
            let max_stamina = player.core_stats.stamina.max_stamina;
            let stamina = (current_stamina / max_stamina) * 100.0;
            stamina_bar.width = Val::Percent(stamina)
        }
    }
}
