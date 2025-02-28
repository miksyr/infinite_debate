use crate::entities::{Effect, Philosopher};

#[derive(Debug)]
pub struct DamageCounter {
    pub current_damage: u8,
}
impl DamageCounter {
    pub fn apply_heal(&mut self, heal: u8) {
        self.current_damage = self.current_damage.saturating_sub(heal);
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.current_damage += damage;
    }
}

#[derive(Debug)]
pub struct InPlayPhilosopher {
    pub philosopher: Philosopher,
    pub current_damage: DamageCounter,
    pub effects: Vec<Effect>,
}
impl InPlayPhilosopher {
    pub fn new(philosopher: Philosopher) -> Self {
        InPlayPhilosopher {
            philosopher,
            current_damage: DamageCounter { current_damage: 0 },
            effects: vec![],
        }
    }

    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn apply_existing_effects(&mut self) {
        for effect in &mut self.effects {
            effect.apply(&mut self.current_damage);
        }
        // Remove effects where duration == 0
        self.effects.retain(|effect| match effect {
            Effect::Poison { duration, .. } | Effect::Recovery { duration, .. } => *duration > 0,
        });
    }

    pub fn apply_heal(&mut self, heal: u8) {
        self.current_damage.apply_heal(heal);
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.current_damage.apply_damage(damage);
    }
}
