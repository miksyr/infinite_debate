use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub enum CoreSchool {
    Rationalist,
    Empiricist,
    Skeptic,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum AbilityType {
    Damage { damage: u8, duration: u8 },
    Heal { heal: u8, duration: u8 },
}

#[derive(Debug)]
pub enum Card {
    Action(Action),
    Philosopher(Philosopher),
    InPlayPhilosopher(InPlayPhilosopher),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    name: String,
    description: String,
    school: CoreSchool,
    pub ability_type: AbilityType,
    additional_effects: Option<Vec<Effect>>,
}
impl Action {
    pub fn new(
        name: String,
        description: String,
        school: CoreSchool,
        ability_type: AbilityType,
        additional_effects: Option<Vec<Effect>>,
    ) -> Self {
        Self {
            name,
            description,
            school,
            ability_type,
            additional_effects,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Philosopher {
    pub name: String,
    pub school: CoreSchool,
    pub starting_health: u8,
}
impl Philosopher {
    pub fn new(name: String, school: CoreSchool, starting_health: u8) -> Self {
        Self {
            name,
            school,
            starting_health,
        }
    }
}

#[derive(Debug)]
struct DamageCounter {
    current_damage: u8,
}
impl DamageCounter {
    pub fn apply_heal(&mut self, heal: u8) {
        self.current_damage = self.current_damage.saturating_sub(heal);
    }

    pub fn apply_damage(&mut self, damage: u8) {
        self.current_damage -= damage;
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
