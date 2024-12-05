use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub size: f32,
    pub speed: f32,
    pub core_stats: CoreStats,
    pub xp: f32,
    pub level: u32,
}

#[derive(Component)]
pub struct CoreStats {
    pub health: f32,
    pub shield: f32,
    pub stamina: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            size: 500.0,
            speed: 500.0,
            core_stats: CoreStats {
                health: 100.0,
                shield: 20.0,
                stamina: 10.0,
            },
            xp: 100.0,
            level: 1,
        }
    }
}

impl Player {
    fn update_level(&mut self) {
        let xp_required_next_level = (self.level as f32) * 1.1;
        if self.xp >= xp_required_next_level {
            self.xp = xp_required_next_level;
            self.level += 1
        }
    }
}
