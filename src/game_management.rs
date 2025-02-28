use super::entities::{AbilityType, Action, Card, Effect, InPlayPhilosopher, Philosopher};
use super::player::{PlayerHand, RemainingDeck};
use rand::{rng, Rng};
use serde_yaml;

#[derive(Debug)]
pub enum GamePhase {
    Player1Turn,
    Player2Turn,
    GameOver,
}

#[derive(Debug)]
pub struct GameBoard {
    pub player_1_hand: PlayerHand,
    pub player_1_deck: RemainingDeck,
    pub player_2_hand: PlayerHand,
    pub player_2_deck: RemainingDeck,
    pub game_phase: GamePhase,
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
            game_phase: GamePhase::Player1Turn,
        }
    }

    pub fn active_player_data(
        &self,
    ) -> Result<(&PlayerHand, &RemainingDeck), Box<dyn std::error::Error>> {
        match self.game_phase {
            GamePhase::Player1Turn => Ok((&self.player_1_hand, &self.player_1_deck)),
            GamePhase::Player2Turn => Ok((&self.player_2_hand, &self.player_2_deck)),
            _ => Err(Into::into("bad game phase")),
        }
    }

    pub fn apply_cards(&mut self, cards: Vec<Card>) {
        let enemy_target: Option<&mut InPlayPhilosopher> = match self.game_phase {
            GamePhase::Player1Turn => self.player_2_hand.active_philosopher.as_mut(),
            GamePhase::Player2Turn => self.player_1_hand.active_philosopher.as_mut(),
            _ => None,
        };
        let friendly_target: Option<&mut InPlayPhilosopher> = match self.game_phase {
            GamePhase::Player1Turn => self.player_1_hand.active_philosopher.as_mut(),
            GamePhase::Player2Turn => self.player_2_hand.active_philosopher.as_mut(),
            _ => None,
        };
        //        for card in cards {
        //            match card {
        //                Card::Action(action) => {
        //                    GameBoard::take_single_action(action, &friendly_target, &enemy_target);
        //                }
        //                Card::Philosopher(_) => continue,
        //                Card::InPlayPhilosopher(_) => continue,
        //            }
        //        }
    }

    fn take_single_action(
        card: Action,
        friendly_target: Option<&mut InPlayPhilosopher>,
        enemy_target: Option<&mut InPlayPhilosopher>,
    ) {
        match card.ability_type {
            AbilityType::Heal { heal, duration } => match friendly_target {
                Some(phil) => {
                    phil.apply_heal(heal);
                    if duration > 0 {
                        phil.add_effect(Effect::Recovery {
                            heal,
                            duration: duration - 1,
                        });
                    }
                }
                None => return,
            },
            AbilityType::Damage { damage, duration } => match enemy_target {
                Some(phil) => {
                    phil.apply_damage(damage);
                    if duration > 0 {
                        phil.add_effect(Effect::Poison {
                            damage,
                            duration: duration - 1,
                        });
                    }
                }
                None => return,
            },
        }
    }

    pub fn next_turn(&mut self) {}
}

fn get_intial_deck() -> Result<(PlayerHand, RemainingDeck), Box<dyn std::error::Error>> {
    let mut philosophers = get_philosopher_cards()?;
    let random_index = rng().random_range(0..philosophers.len());
    let initial_philosopher = philosophers.remove(random_index);
    let actions = get_action_cards()?;
    let mut remaining_deck_cards = philosophers;
    remaining_deck_cards.extend(actions);
    let mut remaining_deck = RemainingDeck::new(remaining_deck_cards);
    let player_initial_cards = remaining_deck.draw_new_cards(4);
    let mut player_hand = PlayerHand {
        active_philosopher: None,
        inactive_cards: vec![initial_philosopher],
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
