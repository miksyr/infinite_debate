use crate::modules::entities;
use rand::seq::SliceRandom;
use rand::{rng, Rng};
use serde_yaml;

#[derive(Debug)]
pub struct PlayerHand {
    active_card: Option<entities::InPlayPhilosopher>,
    inactive_cards: Vec<Box<dyn entities::Card>>,
}

#[derive(Debug)]
pub struct RemainingDeck {
    cards: Vec<Box<dyn entities::Card>>,
}
impl RemainingDeck {
    pub fn new(mut cards: Vec<Box<dyn entities::Card>>) -> Self {
        cards.shuffle(&mut rng());
        RemainingDeck { cards }
    }
    pub fn remaining_cards(&self) -> u8 {
        self.cards.len().try_into().unwrap()
    }
    pub fn draw_new_cards(
        &mut self,
        n: u8,
    ) -> Result<Vec<Box<dyn entities::Card>>, Box<dyn std::error::Error>> {
        let n: u8 = n.min(self.remaining_cards());
        let selected_cards: Vec<Box<dyn entities::Card>> =
            self.cards.drain(0..n as usize).collect();
        Ok(selected_cards)
    }
}

pub fn get_intial_deck() -> Result<(PlayerHand, RemainingDeck), Box<dyn std::error::Error>> {
    let mut philosophers = get_philosopher_cards()?;
    let random_index = rng().random_range(0..philosophers.len());
    let initial_philosopher = philosophers.remove(random_index);
    let player_hand = PlayerHand {
        active_card: None,
        inactive_cards: vec![initial_philosopher],
    };
    let actions = get_action_cards()?;
    let mut remaining_deck_cards = philosophers;
    remaining_deck_cards.extend(actions);

    let remaining_deck = RemainingDeck::new(remaining_deck_cards);
    Ok((player_hand, remaining_deck))
}

fn get_philosopher_cards() -> Result<Vec<Box<dyn entities::Card>>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/philosophers.yaml")?;
    let d: Vec<entities::Philosopher> = serde_yaml::from_reader(f)?;
    let philosopher_cards: Vec<Box<dyn entities::Card>> = d
        .into_iter()
        .map(|p| Box::new(p) as Box<dyn entities::Card>)
        .collect();
    Ok(philosopher_cards)
}

fn get_action_cards() -> Result<Vec<Box<dyn entities::Card>>, Box<dyn std::error::Error>> {
    let f = std::fs::File::open("./assets/actions.yaml")?;
    let d: Vec<entities::Action> = serde_yaml::from_reader(f)?;
    let action_cards: Vec<Box<dyn entities::Card>> = d
        .into_iter()
        .map(|a| Box::new(a) as Box<dyn entities::Card>)
        .collect();
    Ok(action_cards)
}
