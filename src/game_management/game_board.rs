use crate::entities::{AbilityType, Action, Card, Effect, InPlayPhilosopher, Philosopher};
use crate::game_management::helper_functions;
use crate::player::{PlayerHand, RemainingDeck};

#[derive(Debug, PartialEq)]
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

    fn next_phase(&mut self) {
        match self.game_phase {
            GamePhase::Player1Turn => self.game_phase = GamePhase::Player2Turn,
            GamePhase::Player2Turn => self.game_phase = GamePhase::Player1Turn,
            GamePhase::GameOver => self.game_over(),
        }
    }

    fn game_over(&mut self) {
        todo!()
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
                    self.take_single_action(&action);
                }
                Card::Philosopher(p) => self.play_philosopher(Card::Philosopher(p)),
                Card::InPlayPhilosopher(p) => self.play_philosopher(Card::InPlayPhilosopher(p)),
            }
        }
    }

    fn play_philosopher(&mut self, philosopher: Card) {
        todo!()
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

    fn take_single_action(&mut self, card: &Action) {
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
    use crate::test_utils;

    fn get_example_board() -> GameBoard {
        let p1_hand = test_utils::get_populated_player_hand(10);
        let p2_hand = test_utils::get_populated_player_hand(12);
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
    fn test_next_phase() {
        let mut game_board = get_example_board();
        assert_eq!(game_board.game_phase, GamePhase::Player1Turn);
        game_board.next_phase();
        assert_eq!(game_board.game_phase, GamePhase::Player2Turn);
    }

    fn assert_health_after_action(
        game_board: &mut GameBoard,
        action_card: &Action,
        expected_health: u8,
    ) {
        game_board.take_single_action(&action_card);
        let post_action_health = game_board
            .get_target(&action_card.ability_type)
            .as_ref()
            .expect("post-action philosopher not found")
            .remaining_health();
        assert_eq!(post_action_health, expected_health);
    }

    fn unwrap_action_card(card: Card) -> Action {
        if let Card::Action(a) = card {
            a
        } else {
            panic!("Expected 'Action' card")
        }
    }

    #[test]
    fn test_take_single_action_damage_no_duration() {
        let expected_damage = 5;
        let mut game_board = get_example_board();
        let action_card =
            unwrap_action_card(test_utils::get_example_damage_action(expected_damage, 0));
        let target_initial_health = game_board
            .get_target(&action_card.ability_type)
            .as_ref()
            .expect("target philosopher not found")
            .remaining_health();
        assert_health_after_action(
            &mut game_board,
            &action_card,
            target_initial_health - expected_damage,
        );
    }

    #[test]
    fn test_take_single_action_damage_w_duration() {
        let expected_damage = 5;
        let duration = 4;
        let mut game_board = get_example_board();
        let action_card = unwrap_action_card(test_utils::get_example_damage_action(
            expected_damage,
            duration,
        ));
        let target = game_board.get_target(&action_card.ability_type);

        let target_initial_health = target
            .as_ref()
            .expect("target philosopher not found")
            .remaining_health();
        assert_health_after_action(
            &mut game_board,
            &action_card,
            target_initial_health - expected_damage,
        );

        let target = game_board.get_target(&action_card.ability_type).unwrap();
        let num_effects = target.effects.len();
        assert_eq!(num_effects, 1);
        let applied_effect = &target.effects[0];
        if let Effect::Poison {
            damage: ab_damage,
            duration: ab_duration,
        } = *applied_effect
        {
            assert_eq!(ab_damage, expected_damage);
            assert_eq!(ab_duration, duration - 1);
        } else {
            panic!("Ability wasn't Damage/Poison!")
        }
    }

    #[test]
    fn test_take_single_action_heal() {
        let initial_damage = 5;
        let heal = 2;
        let mut game_board = get_example_board();
        let action_card = unwrap_action_card(test_utils::get_example_heal_action(heal, 0));
        let target = game_board.get_target(&action_card.ability_type).unwrap();
        let target_initial_health = target.remaining_health();
        target.apply_direct_damage(initial_damage);
        assert_health_after_action(
            &mut game_board,
            &action_card,
            target_initial_health - initial_damage + heal,
        );
    }

    #[test]
    fn test_take_single_action_heal_duration_full_health() {
        let expected_heal = 6;
        let duration = 2;
        let mut game_board = get_example_board();
        let action_card =
            unwrap_action_card(test_utils::get_example_heal_action(expected_heal, duration));
        let target = game_board.get_target(&action_card.ability_type);
        let target_initial_health = target
            .as_ref()
            .expect("target philosopher not found")
            .remaining_health();
        assert_health_after_action(&mut game_board, &action_card, target_initial_health);

        let target = game_board.get_target(&action_card.ability_type).unwrap();
        let num_effects = target.effects.len();
        assert_eq!(num_effects, 1);
        let applied_effect = &target.effects[0];
        if let Effect::Recovery {
            heal: ab_heal,
            duration: ab_duration,
        } = *applied_effect
        {
            assert_eq!(ab_heal, expected_heal);
            assert_eq!(ab_duration, duration - 1);
        } else {
            panic!("Ability wasn't Damage/Poison!")
        }
    }
}
