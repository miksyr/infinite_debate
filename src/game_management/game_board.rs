use crate::entities::{AbilityType, Action, Card, Effect, InPlayPhilosopher, Philosopher};
use crate::game_management::helper_functions;
use crate::player::{PlayerHand, RemainingDeck};

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
        let (p1_start_hand, p1_deck) =
            helper_functions::get_intial_deck().expect("Can't get player1 hand");
        let (p2_start_hand, p2_deck) =
            helper_functions::get_intial_deck().expect("Can't get player2 hand");
        GameBoard {
            player_1_hand: p1_start_hand,
            player_1_deck: p1_deck,
            player_2_hand: p2_start_hand,
            player_2_deck: p2_deck,
            game_phase: GamePhase::Player1Turn,
        }
    }

    pub fn next_turn(&mut self) {
        todo!()
        // switch phase
        // apply cards
        // apply effects
    }

    pub fn active_player_data(
        &self,
    ) -> Result<(&PlayerHand, &RemainingDeck), Box<dyn std::error::Error>> {
        match self.game_phase {
            GamePhase::Player1Turn => Ok((&self.player_1_hand, &self.player_1_deck)),
            GamePhase::Player2Turn => Ok((&self.player_2_hand, &self.player_2_deck)),
            _ => Err("bad game phase".into()),
        }
    }

    pub fn apply_cards(&mut self, cards: Vec<Card>) {
        for card in cards {
            match card {
                Card::Action(action) => {
                    self.take_single_action(action);
                }
                Card::Philosopher(_) => continue,
                Card::InPlayPhilosopher(_) => continue,
            }
        }
    }

    fn get_target(&mut self, ability_type: &AbilityType) -> Option<&mut InPlayPhilosopher> {
        match ability_type {
            AbilityType::Damage { .. } => match self.game_phase {
                GamePhase::Player1Turn => self.player_2_hand.active_philosopher.as_mut(),
                GamePhase::Player2Turn => self.player_1_hand.active_philosopher.as_mut(),
                _ => None,
            },
            AbilityType::Heal { .. } => match self.game_phase {
                GamePhase::Player1Turn => self.player_1_hand.active_philosopher.as_mut(),
                GamePhase::Player2Turn => self.player_2_hand.active_philosopher.as_mut(),
                _ => None,
            },
        }
    }

    fn take_single_action(&mut self, card: Action) {
        let target = self.get_target(&card.ability_type);

        match card.ability_type {
            AbilityType::Heal { heal, duration } => match target {
                Some(phil) => {
                    phil.apply_direct_heal(heal);
                    if duration > 0 {
                        phil.add_effect(Effect::Recovery {
                            heal,
                            duration: duration - 1,
                        });
                    }
                }
                None => return,
            },
            AbilityType::Damage { damage, duration } => match target {
                Some(phil) => {
                    phil.apply_direct_damage(damage);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_example_damage_action, get_example_heal_action, get_populated_player_hand,
    };

    fn get_example_board() -> GameBoard {
        let p1_hand = get_populated_player_hand(10);
        let p2_hand = get_populated_player_hand(12);
        let game_board = GameBoard {
            player_1_hand: p1_hand,
            player_1_deck: RemainingDeck::new(vec![], None),
            player_2_hand: p2_hand,
            player_2_deck: RemainingDeck::new(vec![], None),
            game_phase: GamePhase::Player1Turn,
        };
        game_board
    }

    #[test]
    fn test_take_single_action_damage() {
        let game_board = get_example_board();
        let action_card = get_example_damage_action(5, 0);
    }
}
