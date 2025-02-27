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
    fn play(&self, target: Card) {
        println!("playing action: {}", &self.name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Philosopher {
    name: String,
    school: CoreSchool,
    starting_health: u8,
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
