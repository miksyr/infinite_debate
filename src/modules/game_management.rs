use crate::modules::entities;
use rand::seq::SliceRandom;
use rand::{rng, Rng};
use serde_yaml;

#[derive(Debug, Default)]
pub struct PlayerHand {
    active_philosopher: Option<entities::InPlayPhilosopher>,
    inactive_cards: Vec<Box<dyn entities::Card>>,
}
impl PlayerHand {
    pub fn add_cards_to_hand(&mut self, cards: Vec<Box<dyn entities::Card>>) {
        self.inactive_cards.extend(cards);
    }

    pub fn play_philosopher(&mut self, philosopher_card: entities::Philosopher) {
        self.active_philosopher = Some(entities::InPlayPhilosopher::new(philosopher_card));
    }
}

#[derive(Debug)]
pub enum PlayerTurn {
    Player1,
    Player2,
}

#[derive(Debug)]
pub struct GameBoard {
    player_1_hand: PlayerHand,
    player_1_deck: RemainingDeck,
    player_2_hand: PlayerHand,
    player_2_deck: RemainingDeck,
    player_turn: PlayerTurn,
}
impl GameBoard {
    pub fn new() -> Self {
        let (p1_start_hand, p1_deck) = get_intial_deck().expect("Can't get player1 hand");
        let (p2_start_hand, p2_deck) = get_intial_deck().expect("Can't get player2 hand");
        GameBoard {
            player_1_hand: p1_start_hand,
            player_1_deck: p1_deck,
            player_2_hand: p2_start_hand,
            player_2_deck: p2_deck,
            player_turn: PlayerTurn::Player1,
        }
    }
}

#[derive(Debug)]
struct RemainingDeck {
    cards: Vec<Box<dyn entities::Card>>,
}
impl RemainingDeck {
    pub fn new(mut cards: Vec<Box<dyn entities::Card>>) -> Self {
        cards.shuffle(&mut rng());
        RemainingDeck { cards }
    }
    pub fn num_remaining_cards(&self) -> u8 {
        self.cards.len().try_into().unwrap()
    }
    pub fn draw_new_cards(
        &mut self,
        n: u8,
    ) -> Result<Vec<Box<dyn entities::Card>>, Box<dyn std::error::Error>> {
        let n: u8 = n.min(self.num_remaining_cards());
        let selected_cards: Vec<Box<dyn entities::Card>> =
            self.cards.drain(0..n as usize).collect();
        Ok(selected_cards)
    }
}

fn get_intial_deck() -> Result<(PlayerHand, RemainingDeck), Box<dyn std::error::Error>> {
    let mut philosophers = get_philosopher_cards()?;
    let random_index = rng().random_range(0..philosophers.len());
    let initial_philosopher = philosophers.remove(random_index);
    let player_hand = PlayerHand {
        active_philosopher: None,
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
