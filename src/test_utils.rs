#[cfg(test)]
use crate::entities::{AbilityType, Action, Card, CoreSchool};
#[cfg(test)]
use crate::entities::{InPlayPhilosopher, Philosopher};
#[cfg(test)]
use crate::player::PlayerHand;

#[cfg(test)]
pub fn get_example_in_play_philosopher(name: String, starting_health: u8) -> InPlayPhilosopher {
    let example_philosopher = Philosopher::new(name, CoreSchool::Rationalist, starting_health);
    InPlayPhilosopher::new(example_philosopher)
}

#[cfg(test)]
pub fn get_populated_player_hand(in_play_philosopher_health: u8) -> PlayerHand {
    let example_philosopher = Philosopher::new(
        "test".into(),
        CoreSchool::Skeptic,
        in_play_philosopher_health,
    );
    let player_hand = PlayerHand {
        active_philosopher: Some(InPlayPhilosopher::new(example_philosopher)),
        inactive_cards: get_example_cards(),
    };
    player_hand
}

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
