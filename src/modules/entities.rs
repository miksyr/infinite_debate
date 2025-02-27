use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
enum CoreSchool {
    Rationalist,
    Empiricist,
    Skeptic,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Effect {
    Poison,
    Weakness,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "PascalCase")]
pub enum AbilityType {
    Damage { damage: u8, duration: u8 },
    Heal { heal: u8, duration: u8 },
}

pub enum CardType {
    Action,
    Philosopher,
    InPlayPhilosopher,
}
pub trait Card: Debug {
    fn card_type(&self) -> CardType;
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
    fn play(&self, target: &dyn Card) {
        println!("playing action: {}", &self.name)
    }
}
impl Card for Action {
    fn card_type(&self) -> CardType {
        CardType::Action
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Philosopher {
    name: String,
    school: CoreSchool,
    starting_health: u8,
}
impl Card for Philosopher {
    fn card_type(&self) -> CardType {
        CardType::Philosopher
    }
}

#[derive(Debug)]
pub struct InPlayPhilosopher {
    pub philosopher: Philosopher,
    pub current_damage: u8,
    pub modifiers: Option<Vec<Effect>>,
}
impl InPlayPhilosopher {
    pub fn new(philosopher: Philosopher) -> Self {
        InPlayPhilosopher {
            philosopher,
            current_damage: 0,
            modifiers: None,
        }
    }
    pub fn apply_heal(&mut self, heal: u8, duration: u8) {
        todo!()
    }
    pub fn apply_damage(&mut self, damage: u8, duration: u8) {
        todo!()
    }
}
impl Card for InPlayPhilosopher {
    fn card_type(&self) -> CardType {
        CardType::InPlayPhilosopher
    }
}
