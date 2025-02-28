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
    pub fn is_expired(&self) -> bool {
        self.duration() == 0
    }
    pub fn duration(&self) -> u8 {
        match self {
            Effect::Poison { duration, .. } | Effect::Recovery { duration, .. } => *duration,
        }
    }
    pub fn magnitude(&self) -> u8 {
        match self {
            Effect::Poison { damage, .. } => *damage,
            Effect::Recovery { heal, .. } => *heal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_damage_effect() {
        let mut damage_counter = DamageCounter { current_damage: 0 };
        let mut damage_effect = Effect::Poison {
            damage: 3,
            duration: 2,
        };
        damage_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 3);
        assert_eq!(damage_effect.magnitude(), 3);
        assert_eq!(damage_effect.duration(), 1);
        assert!(!damage_effect.is_expired());
    }

    #[test]
    fn test_multiple_damage_effect() {
        let mut damage_counter = DamageCounter { current_damage: 0 };
        let mut damage_effect = Effect::Poison {
            damage: 3,
            duration: 2,
        };
        damage_effect.apply(&mut damage_counter);
        damage_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 6);
        assert_eq!(damage_effect.magnitude(), 3);
        assert_eq!(damage_effect.duration(), 0);
        assert!(damage_effect.is_expired());
    }

    #[test]
    fn test_expired_damage_effect() {
        let mut damage_counter = DamageCounter { current_damage: 0 };
        let mut damage_effect = Effect::Poison {
            damage: 3,
            duration: 0,
        };
        assert!(damage_effect.is_expired());
        damage_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 0);
        assert_eq!(damage_effect.magnitude(), 3);
        assert_eq!(damage_effect.duration(), 0);
        assert!(damage_effect.is_expired());
    }

    #[test]
    fn test_single_heal_effect() {
        let mut damage_counter = DamageCounter { current_damage: 10 };
        let mut heal_effect = Effect::Recovery {
            heal: 5,
            duration: 3,
        };
        heal_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 5);
        assert_eq!(heal_effect.magnitude(), 5);
        assert_eq!(heal_effect.duration(), 2);
        assert!(!heal_effect.is_expired());
    }

    #[test]
    fn test_multiple_heal_effect() {
        let mut damage_counter = DamageCounter { current_damage: 9 };
        let mut heal_effect = Effect::Recovery {
            heal: 3,
            duration: 2,
        };
        heal_effect.apply(&mut damage_counter);
        heal_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 3);
        assert_eq!(heal_effect.magnitude(), 3);
        assert_eq!(heal_effect.duration(), 0);
        assert!(heal_effect.is_expired());
    }

    #[test]
    fn test_expired_heal_effect() {
        let mut damage_counter = DamageCounter { current_damage: 9 };
        let mut heal_effect = Effect::Recovery {
            heal: 3,
            duration: 0,
        };
        assert!(heal_effect.is_expired());
        heal_effect.apply(&mut damage_counter);
        assert_eq!(damage_counter.current_damage, 9);
        assert_eq!(heal_effect.magnitude(), 3);
        assert_eq!(heal_effect.duration(), 0);
        assert!(heal_effect.is_expired());
    }
}
