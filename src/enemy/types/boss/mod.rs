use crate::enemy::xp::XP;

pub enum BossEnemy {
    Mage,
    Dragon,
}

impl BossEnemy {
    pub fn stats(&self) -> (f32, f32, f32, f32, u32, XP) {
        match *self {
            BossEnemy::Mage => (
                2000.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 1000.0,
                    drop_chance: 1.0,
                    size: 10.0,
                },
            ),
            BossEnemy::Dragon => (
                5000.0,
                150.0,
                300.0,
                60.0,
                1,
                XP {
                    xp: 1200.0,
                    drop_chance: 1.0,
                    size: 10.0,
                },
            ),
        }
    }
}
