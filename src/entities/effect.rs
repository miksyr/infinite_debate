use crate::entities::DamageCounter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Effect {
    Poison { damage: u8, duration: u8 },
    Recovery { heal: u8, duration: u8 },
}
impl Effect {
    pub fn apply(&mut self, target: &mut DamageCounter) {
        match self {
            Effect::Poison { damage, duration } => {
                if *duration > 0 {
                    target.apply_damage(*damage);
                }
                *duration = duration.saturating_sub(1);
            }
            Effect::Recovery { heal, duration } => {
                if *duration > 0 {
                    target.apply_heal(*heal);
                }
                *duration = duration.saturating_sub(1);
            }
        }
    }
}
