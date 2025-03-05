use crate::entities::Effect;
use crate::entities::InPlayPhilosopher;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CoreSchool {
    Rationalist,
    Empiricist,
    Skeptic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum AbilityType {
    Damage { damage: u8, duration: u8 },
    Heal { heal: u8, duration: u8 },
}

#[derive(Clone, Debug)]
pub enum Card {
    Action(Action),
    Philosopher(Philosopher),
    InPlayPhilosopher(InPlayPhilosopher),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
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
