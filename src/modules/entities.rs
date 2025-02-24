use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fmt::Debug;

pub trait Card: Debug {
    fn get_name(&self) -> &str;
    // fn get_cost(&self) -> u32;
    fn play(&self);
}

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
enum AbilityType {
    Damage { damage: u8 },
    DamageOverTime { damage: u8, duration: u8 },
    Heal { heal: u8 },
    HealOverTime { heal: u8, duration: u8 },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    name: String,
    description: String,
    school: CoreSchool,
    ability_type: AbilityType,
    additional_effects: Option<Vec<Effect>>,
}
impl Card for Action {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn play(&self) {
        println!("playing action: {}", &self.name)
    }
}

pub fn get_actions() -> Result<Vec<Action>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/actions.yaml")?;
    let actions = serde_yaml::from_reader(f)?;
    Ok(actions)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Philosopher {
    name: String,
    school: CoreSchool,
    starting_health: u8,
    // base_defence: u8,
    // base_attack: u8,
}
impl Card for Philosopher {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn play(&self) {
        println!("playing philosopher: {:?}", &self.name)
        // this should produce an InPlayPhilosopher
    }
}

pub fn get_philosopher_set() -> Result<Vec<Philosopher>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/philosophers.yaml")?;
    let d: Vec<Philosopher> = serde_yaml::from_reader(f)?;
    Ok(d)
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

    pub fn with_state(
        philosopher: Philosopher,
        current_damage: u8,
        modifiers: Option<Vec<Effect>>,
    ) -> Self {
        InPlayPhilosopher {
            philosopher,
            current_damage,
            modifiers,
        }
    }
}

impl Card for InPlayPhilosopher {
    fn get_name(&self) -> &str {
        &self.philosopher.name
    }
    fn play(&self) {
        println!("philosopher already in play {:?}", &self.philosopher.name)
        // this should be a default action?
    }
}
