use crate::modules::entities;
use rand::seq::SliceRandom;
use rand::{rng, Rng};

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
        RemainingDeck { cards: cards }
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
    let mut philosophers = entities::get_philosopher_set()?;
    let random_index = rng().random_range(0..philosophers.len());
    let initial_philosopher = philosophers.remove(random_index);
    let player_hand = PlayerHand {
        active_card: None,
        inactive_cards: vec![Box::new(initial_philosopher)],
    };
    // let player1 = entities::InPlayPhilosopher::new(chosen_philosopher);
    //let actions: Vec<Action> = entities::get_actions();
    let remaining_philosophers: Vec<Box<dyn entities::Card>> = philosophers
        .into_iter()
        .map(|p| Box::new(p) as Box<dyn entities::Card>)
        .collect();
    let remaining_deck = RemainingDeck {
        cards: remaining_philosophers,
    };
    Ok((player_hand, remaining_deck))
}
