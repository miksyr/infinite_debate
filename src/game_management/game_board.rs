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
            AbilityType::Damage { damage, duration } => match enemy_target {
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

    pub fn next_turn(&mut self) {
        todo!()
        // switch phase
        // apply cards
        // apply effects
    }
}
