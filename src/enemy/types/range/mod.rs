use crate::enemy::xp::XP;

pub enum RangeEnemy {
    Archer,
    Gunner,
    Wizard,
    Marksman,
}

impl RangeEnemy {
    pub fn stats(&self) -> (f32, f32, f32, f32, u32, XP) {
        match *self {
            RangeEnemy::Archer => (
                100.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 15.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
            RangeEnemy::Gunner => (
                100.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 15.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
            RangeEnemy::Wizard => (
                100.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 25.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
            RangeEnemy::Marksman => (
                150.0,
                150.0,
                300.0,
                60.0,
                1,
                XP {
                    xp: 40.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
        }
    }
}
