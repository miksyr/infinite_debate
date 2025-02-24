use crate::modules::entities;
use rand::Rng;

#[derive(Debug)]
pub struct PlayerHand {
    active_card: Option<entities::InPlayPhilosopher>,
    inactive_cards: Vec<Box<dyn entities::Card>>,
}

#[derive(Debug)]
pub struct RemainingDeck {
    cards: Vec<Box<dyn entities::Card>>,
}

pub fn get_intial_deck() -> Result<(PlayerHand, RemainingDeck), Box<dyn std::error::Error>> {
    let mut philosophers = entities::get_philosopher_set()?;
    let random_index = rand::rng().random_range(0..philosophers.len());
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
