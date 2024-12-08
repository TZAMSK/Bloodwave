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
    pub health: Health,
    pub shield: Shield,
    pub stamina: Stamina,
}

#[derive(Component)]
pub struct Health {
    pub current_health: f32,
    pub max_health: f32,
}

#[derive(Component)]
pub struct Shield {
    pub current_shield: f32,
    pub max_shield: f32,
}

#[derive(Component)]
pub struct Stamina {
    pub current_stamina: f32,
    pub max_stamina: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            size: 500.0,
            speed: 500.0,
            core_stats: CoreStats {
                health: Health {
                    current_health: 100.0,
                    max_health: 100.0,
                },
                shield: Shield {
                    current_shield: 20.0,
                    max_shield: 20.0,
                },
                stamina: Stamina {
                    current_stamina: 10.0,
                    max_stamina: 10.0,
                },
            },
            xp: 0.0,
            level: 1,
        }
    }
}

impl Player {
    pub fn xp_progress(&mut self) -> f32 {
        let xp_required_next_level = (self.level as f32) * 100.0 * 1.2;
        if self.xp >= xp_required_next_level {
            self.xp = 0.0;
            self.level += 1;
        }
        self.xp / xp_required_next_level
    }
}
