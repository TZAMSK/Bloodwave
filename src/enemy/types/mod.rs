pub mod boss;
pub mod melee;
pub mod range;
pub mod support;

use boss::BossEnemy;
use melee::MeleeEnemy;
use range::RangeEnemy;
use support::SupportEnemy;

pub enum EnemyTypes {
    Melee(MeleeEnemy),
    Range(RangeEnemy),
    Support(SupportEnemy),
    Boss(BossEnemy),
}
