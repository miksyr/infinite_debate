use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Deserialize, Serialize)]
enum CoreSchool {
    Rationalist,
    Empiricist,
    Skeptic,
}

#[derive(Debug, Deserialize, Serialize)]
enum Effect {
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
struct Action {
    name: String,
    description: String,
    ability_type: AbilityType,
    additional_effects: Option<Vec<Effect>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Philosopher {
    name: String,
    school: CoreSchool,
    starting_health: u8,
    action_set: Vec<Action>,
    // defence: u8,
    // attack: u8,
}

pub fn get_philosopher_set() -> Result<Vec<Philosopher>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/philosophers.yaml")?;
    let d: Vec<Philosopher> = serde_yaml::from_reader(f)?;
    Ok(d)
}

pub struct InPlayPhilosopher {
    philosopher: Philosopher,
    current_damage: u8,
    modifiers: Option<Vec<Effect>>,
}
