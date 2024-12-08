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
                        frame.spawn(TextBundle {
                            text: Text::from_section(
                                "?",
                                TextStyle {
                                    font: asset_server.load("fonts/Pixel.ttf"),
                                    font_size: 70.0,
                                    color: Color::srgb(192.0 / 255.0, 143.0 / 255.0, 88.0 / 255.0),
                                },
                            ),
                            style: Style { ..default() },
                            ..default()
                        });
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
                    right.spawn(ImageBundle {
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
                    });
                    // Shield
                    right.spawn(ImageBundle {
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
                    });
                    // Stamina
                    right.spawn(ImageBundle {
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
                    });
                });
        });
}
