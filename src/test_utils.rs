#[cfg(test)]
use crate::entities::{AbilityType, Action, Card, CoreSchool, Philosopher};

#[cfg(test)]
pub fn get_example_cards() -> Vec<Box<Card>> {
    let cards = vec![
        Card::Action(Action::new(
            "test_dam".into(),
            "test_desc".into(),
            CoreSchool::Skeptic,
            AbilityType::Damage {
                damage: 1,
                duration: 0,
            },
            None,
        )),
        Card::Action(Action::new(
            "test_heal".into(),
            "test_desc2".into(),
            CoreSchool::Rationalist,
            AbilityType::Heal {
                heal: 3,
                duration: 2,
            },
            None,
        )),
        Card::Philosopher(Philosopher::new(
            "hand_philos".into(),
            CoreSchool::Empiricist,
            6,
        )),
    ];
    let cards: Vec<Box<Card>> = cards.into_iter().map(|card| Box::new(card)).collect();
    cards
}
