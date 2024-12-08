use bevy::prelude::*;

pub fn xp_bar_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Main Node for stats
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                flex_direction: FlexDirection::Row,
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: UiImage {
                    texture: asset_server.load("sprites/hud/xp/xp.png"),
                    ..default()
                },
                style: Style {
                    width: Val::Px(1920.0),
                    height: Val::Px(20.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            });
        });
}
