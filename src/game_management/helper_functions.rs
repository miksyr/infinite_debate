use crate::entities::{Action, Card, Philosopher};
use crate::player::{PlayerHand, RemainingDeck};
use rand::{rng, Rng};
use serde_yaml;

pub fn get_intial_deck(
    max_cards_in_hand: &u8,
) -> Result<(PlayerHand, RemainingDeck), Box<dyn std::error::Error>> {
    let mut philosophers = get_philosopher_cards()?;
    let random_index = rng().random_range(0..philosophers.len());
    let initial_philosopher = philosophers.remove(random_index);
    let actions = get_action_cards()?;
    let mut remaining_deck_cards = philosophers;
    remaining_deck_cards.extend(actions);
    let mut remaining_deck = RemainingDeck::new(remaining_deck_cards, None);
    let player_initial_cards = remaining_deck.draw_new_cards(4);
    let mut player_hand = PlayerHand {
        active_philosopher: None,
        inactive_cards: vec![initial_philosopher],
        max_cards_in_hand: *max_cards_in_hand,
    };
    player_hand.add_cards_to_hand(player_initial_cards?)?;
    Ok((player_hand, remaining_deck))
}

fn get_philosopher_cards() -> Result<Vec<Box<Card>>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/philosophers.yaml")?;
    let d: Vec<Philosopher> = serde_yaml::from_reader(f)?;
    let philosopher_cards: Vec<Box<Card>> = d
        .into_iter()
        .map(|card| Box::new(Card::Philosopher(card)))
        .collect();
    Ok(philosopher_cards)
}

fn get_action_cards() -> Result<Vec<Box<Card>>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/actions.yaml")?;
    let d: Vec<Action> = serde_yaml::from_reader(f)?;
    let action_cards: Vec<Box<Card>> = d
        .into_iter()
        .map(|card| Box::new(Card::Action(card)))
        .collect();
    Ok(action_cards)
}
