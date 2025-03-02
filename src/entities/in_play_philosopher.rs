use crate::entities::{Effect, Philosopher};

#[derive(Debug)]
pub struct DamageCounter {
    pub damage_counter: u8,
}
impl DamageCounter {
    pub fn apply_heal(&mut self, heal: u8) {
        self.damage_counter = self.damage_counter.saturating_sub(heal);
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.damage_counter += damage;
    }
}

#[derive(Debug, PartialEq)]
enum DeathStatus {
    Dead,
    Alive,
}

#[derive(Debug)]
pub struct InPlayPhilosopher {
    pub philosopher: Philosopher,
    pub damage_counter: DamageCounter,
    pub effects: Vec<Effect>,
    pub death_status: DeathStatus,
}
impl InPlayPhilosopher {
    pub fn new(philosopher: Philosopher) -> Self {
        InPlayPhilosopher {
            philosopher,
            damage_counter: DamageCounter { damage_counter: 0 },
            effects: vec![],
            death_status: DeathStatus::Alive,
        }
    }

    pub fn remaining_health(&self) -> u8 {
        self.philosopher
            .starting_health
            .saturating_sub(self.damage_counter.damage_counter)
    }

    pub fn is_dead(&self) -> bool {
        self.remaining_health() == 0
    }

    fn update_death(&mut self) {
        if self.is_dead() {
            self.death_status = DeathStatus::Dead;
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn apply_existing_effects(&mut self) {
        for effect in &mut self.effects {
            effect.apply(&mut self.damage_counter);
            if self.death_status == DeathStatus::Dead {
                break;
            }
        }
        self.update_death();
        // Remove effects where duration == 0
        self.effects.retain(|effect| match effect {
            Effect::Poison { duration, .. } | Effect::Recovery { duration, .. } => *duration > 0,
        });
    }

    pub fn apply_direct_heal(&mut self, heal: u8) {
        if self.death_status == DeathStatus::Alive {
            self.damage_counter.apply_heal(heal);
        }
    }

    pub fn apply_direct_damage(&mut self, damage: u8) {
        self.damage_counter.apply_damage(damage);
        self.update_death();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_example_in_play_philosopher;

    // damage counter tests
    #[test]
    fn test_damage_counter_0() {
        let mut damage_counter = DamageCounter { damage_counter: 0 };
        for i in [1, 2, 3] {
            damage_counter.apply_damage(i);
        }
        for i in [1, 3] {
            damage_counter.apply_heal(i);
        }
        assert_eq!(damage_counter.damage_counter, 2);
    }

    #[test]
    fn test_damage_counter_1() {
        let mut damage_counter = DamageCounter { damage_counter: 0 };
        damage_counter.apply_heal(10);
        for i in [1, 2, 3] {
            damage_counter.apply_damage(i);
        }
        for i in [1, 3] {
            damage_counter.apply_heal(i);
        }
        assert_eq!(damage_counter.damage_counter, 2);
    }

    // in play philosopher tests
    #[test]
    fn test_effect_application() {
        let starting_health = 10;
        let mut philos = get_example_in_play_philosopher("test".into(), starting_health);
        philos.add_effect(Effect::Poison {
            damage: 3,
            duration: 2,
        });
        philos.add_effect(Effect::Recovery {
            heal: 1,
            duration: 1,
        });
        assert_eq!(philos.effects.len(), 2);
        philos.apply_existing_effects();
        assert_eq!(philos.remaining_health(), 8);
        assert_eq!(philos.effects.len(), 1);
        philos.apply_existing_effects();
        assert_eq!(philos.remaining_health(), 5);
    }

    #[test]
    fn test_effect_application_after_death() {
        let starting_health = 3;
        let mut philos = get_example_in_play_philosopher("test".into(), starting_health);
        philos.add_effect(Effect::Poison {
            damage: 10,
            duration: 2,
        });
        philos.add_effect(Effect::Recovery {
            heal: 1,
            duration: 1,
        });
        assert_eq!(philos.effects.len(), 2);
        philos.apply_existing_effects();
        assert_eq!(philos.remaining_health(), 0);
        assert!(philos.is_dead());
        assert_eq!(philos.death_status, DeathStatus::Dead);
    }

    #[test]
    fn test_death() {
        let starting_health = 2;
        let mut philos = get_example_in_play_philosopher("test".into(), starting_health);
        philos.apply_direct_damage(10);
        assert!(philos.is_dead());
        philos.apply_direct_heal(10);
        assert!(philos.is_dead());
        assert_eq!(philos.remaining_health(), 0)
    }
}
