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
pub struct GameConfig {
    num_cards_played_per_turn: u8,
    num_cards_drawn_per_turn: u8,
    max_cards_in_hand: u8,
}
impl GameConfig {
    pub fn max_cards_in_hand(&self) -> u8 {
        self.max_cards_in_hand
    }
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            num_cards_played_per_turn: 3,
            num_cards_drawn_per_turn: 2,
            max_cards_in_hand: 7,
        }
    }
}

#[derive(Debug)]
pub struct GameBoard {
    pub player_1_hand: PlayerHand,
    pub player_1_deck: RemainingDeck,
    pub player_2_hand: PlayerHand,
    pub player_2_deck: RemainingDeck,
    pub game_phase: GamePhase,
    pub game_config: GameConfig,
}
impl GameBoard {
    pub fn new(game_config: Option<GameConfig>) -> Self {
        let config: GameConfig = game_config.unwrap_or_default();
        let (p1_start_hand, p1_deck) = helper_functions::get_intial_deck(&config.max_cards_in_hand)
            .expect("Can't get player1 hand");
        let (p2_start_hand, p2_deck) = helper_functions::get_intial_deck(&config.max_cards_in_hand)
            .expect("Can't get player2 hand");
        GameBoard {
            player_1_hand: p1_start_hand,
            player_1_deck: p1_deck,
            player_2_hand: p2_start_hand,
            player_2_deck: p2_deck,
            game_phase: GamePhase::Player1Turn,
            game_config: config,
        }
    }

    fn update_game_phase(&mut self) {
        match self.game_phase {
            GamePhase::Player1Turn => self.game_phase = GamePhase::Player2Turn,
            GamePhase::Player2Turn => self.game_phase = GamePhase::Player1Turn,
            GamePhase::GameOver => self.game_over(),
        }
    }

    fn game_over(&mut self) {
        todo!()
    }

    fn apply_effects(&mut self) {
        let (inactive_hand, _inactive_deck) = self
            .inactive_player_data()
            .expect("can't get inactive player for applying effects");
        if let Some(p) = inactive_hand.active_philosopher.as_mut() {
            p.apply_existing_effects();
        }
    }

    fn draw_cards_for_next_player(&mut self) {
        // extract value below to avoid borrow checker issues with mutable references to self
        let num_cards_per_turn = self.game_config.num_cards_drawn_per_turn;
        let (inactive_hand, inactive_deck) = self
            .inactive_player_data()
            .expect("can't get inactive player for drawing cards");
        let num_cards_to_draw = num_cards_per_turn.min(inactive_hand.num_available_slots_in_hand());
        let new_cards = inactive_deck
            .draw_new_cards(num_cards_to_draw)
            .expect("couldn't draw new cards for next player");
        let _ = inactive_hand.add_cards_to_hand(new_cards);
    }

    pub fn process_turn(&mut self, cards: Vec<Card>) -> Result<(), Box<dyn std::error::Error>> {
        self.apply_effects();
        self.apply_cards(cards)?;
        self.draw_cards_for_next_player();
        self.update_game_phase();
        Ok(())
    }

    pub fn active_player_data(
        &mut self,
    ) -> Result<(&mut PlayerHand, &mut RemainingDeck), Box<dyn std::error::Error>> {
        match self.game_phase {
            GamePhase::Player1Turn => Ok((&mut self.player_1_hand, &mut self.player_1_deck)),
            GamePhase::Player2Turn => Ok((&mut self.player_2_hand, &mut self.player_2_deck)),
            _ => Err("bad game phase".into()),
        }
    }

    pub fn inactive_player_data(
        &mut self,
    ) -> Result<(&mut PlayerHand, &mut RemainingDeck), Box<dyn std::error::Error>> {
        match self.game_phase {
            GamePhase::Player1Turn => Ok((&mut self.player_2_hand, &mut self.player_2_deck)),
            GamePhase::Player2Turn => Ok((&mut self.player_1_hand, &mut self.player_1_deck)),
            _ => Err("bad game phase".into()),
        }
    }

    pub fn apply_cards(&mut self, cards: Vec<Card>) -> Result<(), Box<dyn std::error::Error>> {
        for card in cards {
            match card {
                Card::Action(action) => self.take_single_action(&action),
                Card::Philosopher(p) => self.play_philosopher(Card::Philosopher(p)),
                Card::InPlayPhilosopher(p) => self.play_philosopher(Card::InPlayPhilosopher(p)),
            }?
        }
        Ok(())
    }

    fn play_philosopher(&mut self, philosopher: Card) -> Result<(), Box<dyn std::error::Error>> {
        let (active_player_hand, _active_player_deck) = self.active_player_data()?;
        active_player_hand.play_philosopher(philosopher)?;
        Ok(())
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

    fn take_single_action(&mut self, card: &Action) -> Result<(), Box<dyn std::error::Error>> {
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
                    Ok(())
                }
                None => Ok(()),
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
                    Ok(())
                }
                None => Ok(()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::CoreSchool, test_utils};
    use uuid::Uuid;

    fn get_example_board() -> GameBoard {
        let p1_hand = test_utils::get_populated_player_hand(10);
        let p2_hand = test_utils::get_populated_player_hand(12);
        let game_board = GameBoard {
            player_1_hand: p1_hand,
            player_1_deck: RemainingDeck::new(vec![], None),
            player_2_hand: p2_hand,
            player_2_deck: RemainingDeck::new(vec![], None),
            game_phase: GamePhase::Player1Turn,
            game_config: GameConfig::default(),
        };
        game_board
    }

    #[test]
    fn test_next_phase() {
        let mut game_board = get_example_board();
        assert_eq!(game_board.game_phase, GamePhase::Player1Turn);
        game_board.update_game_phase();
        assert_eq!(game_board.game_phase, GamePhase::Player2Turn);
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
            .expect("target philosopher not found")
            .remaining_health();
        let _ = game_board.take_single_action(&action_card);
        let post_action_health = game_board
            .get_target(&action_card.ability_type)
            .expect("post-action philosopher not found")
            .remaining_health();
        assert_eq!(post_action_health, target_initial_health - expected_damage);
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
        let target = game_board
            .get_target(&action_card.ability_type)
            .expect("target philosopher not found");
        let target_initial_health = target.remaining_health();
        let _ = game_board.take_single_action(&action_card);
        let post_action_target = game_board
            .get_target(&action_card.ability_type)
            .expect("post-action philosopher not found");
        assert_eq!(
            post_action_target.remaining_health(),
            target_initial_health - expected_damage
        );
        assert_eq!(post_action_target.effects.len(), 1);
        let applied_effect = &post_action_target.effects[0];
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
        let _ = game_board.take_single_action(&action_card);
        let post_action_health = game_board
            .get_target(&action_card.ability_type)
            .expect("post-action philosopher not found")
            .remaining_health();
        assert_eq!(
            post_action_health,
            target_initial_health - initial_damage + heal
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
            .expect("target philosopher not found")
            .remaining_health();
        let _ = game_board.take_single_action(&action_card);
        let post_action_target = game_board
            .get_target(&action_card.ability_type)
            .expect("post-action philosopher not found");
        assert_eq!(post_action_target.remaining_health(), target_initial_health);
        assert_eq!(post_action_target.effects.len(), 1);
        let applied_effect = &post_action_target.effects[0];
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

    #[test]
    fn test_apply_cards_damage_and_then_play_philosopher() {
        let expected_damage = 3;
        let expected_philosopher_name = Uuid::new_v4().to_string();
        let mut game_board = get_example_board();
        let action_card =
            unwrap_action_card(test_utils::get_example_damage_action(expected_damage, 3));
        let ability_type = action_card.ability_type.clone();
        let target = game_board
            .get_target(&ability_type)
            .expect("target philosopher not found");
        let target_initial_health = target.remaining_health();
        let cards = vec![
            Card::Action(action_card),
            Card::Philosopher(Philosopher::new(
                expected_philosopher_name.clone(),
                CoreSchool::Skeptic,
                13,
            )),
        ];
        let _ = game_board.apply_cards(cards);
        let post_action_target = game_board
            .get_target(&ability_type)
            .expect("target philosopher not found");
        assert_eq!(
            post_action_target.remaining_health(),
            target_initial_health - expected_damage
        );
        let (active_player_hand, _d) = game_board.active_player_data().unwrap();
        let active_philosopher = active_player_hand.active_philosopher.as_ref().unwrap();
        assert_eq!(
            active_philosopher.philosopher.name,
            expected_philosopher_name
        );
    }

    #[test]
    fn test_apply_cards_in_play_philosopher() {
        let mut game_board = get_example_board();
        let expected_name = Uuid::new_v4().to_string();
        let cards = vec![Card::InPlayPhilosopher(
            test_utils::get_example_in_play_philosopher(expected_name.clone(), 3),
        )];
        let _ = game_board.apply_cards(cards);
        let (active_player_hand, _d) = game_board.active_player_data().unwrap();
        let active_philosopher = active_player_hand.active_philosopher.as_ref().unwrap();
        assert_eq!(active_philosopher.philosopher.name, expected_name)
    }

    #[test]
    fn test_apply_cards_no_cards() {
        let mut game_board = get_example_board();
        let game_board_repr = format!("{:?}", game_board);
        let cards = vec![];
        let _ = game_board.apply_cards(cards);
        assert_eq!(format!("{:?}", game_board), game_board_repr)
    }
}
