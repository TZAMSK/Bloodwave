use crate::enemy::xp::XP;

pub enum SupportEnemy {
    Healer,
    Buffer,
    Shield,
}

impl SupportEnemy {
    pub fn stats(&self) -> (f32, f32, f32, f32, u32, XP) {
        match *self {
            SupportEnemy::Healer => (
                100.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 60.0,
                    drop_chance: 1.0,
                    size: 10.0,
                },
            ),
            SupportEnemy::Buffer => (
                100.0,
                250.0,
                200.0,
                50.0,
                1,
                XP {
                    xp: 100.0,
                    drop_chance: 1.0,
                    size: 10.0,
                },
            ),
            SupportEnemy::Shield => (
                150.0,
                150.0,
                300.0,
                60.0,
                1,
                XP {
                    xp: 100.0,
                    drop_chance: 1.0,
                    size: 10.0,
                },
            ),
        }
    }
}
