use crate::enemy::xp::XP;

pub enum MeleeEnemy {
    Spider,
    Knight,
}

impl MeleeEnemy {
    pub fn description(&self) -> &str {
        match *self {
            MeleeEnemy::Spider => "A fast creature that poisins over time",
            MeleeEnemy::Knight => "A heavily armored warrior carrying a sword",
        }
    }

    pub fn asset(&self) -> &str {
        match *self {
            MeleeEnemy::Spider => "spider.png",
            MeleeEnemy::Knight => "knight.png",
        }
    }

    pub fn stats(&self) -> (f32, f32, f32, f32, u32, XP) {
        match *self {
            MeleeEnemy::Spider => (
                100.0,
                250.0,
                500.0,
                50.0,
                1,
                XP {
                    xp: 10.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
            MeleeEnemy::Knight => (
                150.0,
                150.0,
                300.0,
                60.0,
                1,
                XP {
                    xp: 20.0,
                    drop_chance: 0.5,
                    size: 10.0,
                },
            ),
        }
    }
}
