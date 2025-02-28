#[cfg(test)]
use crate::entities::Philosopher;
#[cfg(test)]
use crate::entities::{AbilityType, Action, Card, CoreSchool};

#[cfg(test)]
pub fn get_example_damage_action(damage: u8, duration: u8) -> Card {
    Card::Action(Action::new(
        "test_dam".into(),
        "damage_desc".into(),
        CoreSchool::Skeptic,
        AbilityType::Damage { damage, duration },
        None,
    ))
}

#[cfg(test)]
pub fn get_example_heal_action(heal: u8, duration: u8) -> Card {
    Card::Action(Action::new(
        "test_heal".into(),
        "heal_desc".into(),
        CoreSchool::Rationalist,
        AbilityType::Heal { heal, duration },
        None,
    ))
}

#[cfg(test)]
pub fn get_example_cards() -> Vec<Box<Card>> {
    let cards = vec![
        get_example_damage_action(1, 0),
        get_example_heal_action(3, 2),
        Card::Philosopher(Philosopher::new(
            "hand_philos".into(),
            CoreSchool::Empiricist,
            6,
        )),
    ];
    let cards: Vec<Box<Card>> = cards.into_iter().map(|card| Box::new(card)).collect();
    cards
}
