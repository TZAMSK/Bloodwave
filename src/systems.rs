use bevy::ecs::prelude::*;
use bevy::prelude::*;

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_write: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_write.send(AppExit::Success);
    }
}
