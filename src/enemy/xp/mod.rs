use bevy::prelude::Component;

#[derive(Component)]
pub struct XP {
    pub xp: f32,
    pub drop_chance: f32,
    pub size: f32,
}
