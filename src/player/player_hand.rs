use crate::entities::{Card, InPlayPhilosopher};

#[derive(Debug, Default)]
pub struct PlayerHand {
    pub active_philosopher: Option<InPlayPhilosopher>,
    pub inactive_cards: Vec<Box<Card>>,
    pub max_cards_in_hand: u8,
}
impl PlayerHand {
    pub fn add_cards_to_hand(
        &mut self,
        cards: Vec<Box<Card>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.inactive_cards.extend(cards);
        Ok(())
    }

    pub fn play_philosopher(
        &mut self,
        philosopher_card: Card,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(active_philosopher) = self.active_philosopher.take() {
            self.inactive_cards
                .push(Box::new(Card::InPlayPhilosopher(active_philosopher)));
        }
        match philosopher_card {
            Card::Philosopher(philosopher) => {
                self.active_philosopher = Some(InPlayPhilosopher::new(philosopher));
                return Ok(());
            }
            Card::InPlayPhilosopher(in_play_philos) => {
                self.active_philosopher = Some(in_play_philos);
                return Ok(());
            }
            Card::Action(_) => return Err("Action card played as philosopher".into()),
        }
    }

    pub fn num_available_slots_in_hand(&self) -> u8 {
        self.max_cards_in_hand.saturating_sub(
            self.inactive_cards
                .len()
                .try_into()
                .expect("can't get num available slots in hand"),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::{CoreSchool, Philosopher};

    use super::*;
    use crate::test_utils::{get_example_cards, get_populated_player_hand};

    #[test]
    fn test_player_hand_creation_no_cards() {
        let player_hand = PlayerHand {
            active_philosopher: None,
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        assert!(player_hand.active_philosopher.is_none());
        assert_eq!(player_hand.inactive_cards.len(), 0)
    }

    #[test]
    fn test_player_hand_creation_philosopher_card() {
        let player_hand = get_populated_player_hand(16);
        assert!(player_hand.active_philosopher.is_some());
        assert_eq!(player_hand.inactive_cards.len(), 3)
    }

    #[test]
    fn play_philosopher_with_empty_spot() {
        let mut player_hand = PlayerHand {
            active_philosopher: None,
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        let example_philosopher = Philosopher::new("test".into(), CoreSchool::Skeptic, 16);
        let result = player_hand.play_philosopher(Card::Philosopher(example_philosopher));
        assert!(result.is_ok());
        assert!(player_hand.active_philosopher.is_some());
        assert_eq!(
            player_hand.active_philosopher.unwrap().philosopher.name,
            "test"
        );
    }

    #[test]
    fn play_in_play_philosopher_with_empty_spot() {
        let mut player_hand = PlayerHand {
            active_philosopher: None,
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        let example_philosopher = Philosopher::new("test".into(), CoreSchool::Skeptic, 16);
        let ex_in_play_philos = InPlayPhilosopher::new(example_philosopher);
        let result = player_hand.play_philosopher(Card::InPlayPhilosopher(ex_in_play_philos));
        assert!(result.is_ok());
        assert!(player_hand.active_philosopher.is_some());
        assert_eq!(
            player_hand.active_philosopher.unwrap().philosopher.name,
            "test"
        );
    }

    #[test]
    fn replace_in_play_philosopher_with_another_in_play() {
        let example_philos1 = Philosopher::new("first_test".into(), CoreSchool::Rationalist, 2);
        let ex_in_play_philos1 = InPlayPhilosopher::new(example_philos1);
        let mut player_hand = PlayerHand {
            active_philosopher: Some(ex_in_play_philos1),
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        let new_philos = Philosopher::new("expected".into(), CoreSchool::Skeptic, 16);
        let expected_in_play_philos = InPlayPhilosopher::new(new_philos);
        let result = player_hand.play_philosopher(Card::InPlayPhilosopher(expected_in_play_philos));
        assert!(result.is_ok());
        assert_eq!(
            player_hand.active_philosopher.unwrap().philosopher.name,
            "expected"
        );
        assert_ne!(player_hand.inactive_cards.len(), 0)
    }

    #[test]
    fn replace_in_play_philosopher_with_new_philosopher() {
        let example_philos1 = Philosopher::new("first_test".into(), CoreSchool::Rationalist, 2);
        let ex_in_play_philos1 = InPlayPhilosopher::new(example_philos1);
        let mut player_hand = PlayerHand {
            active_philosopher: Some(ex_in_play_philos1),
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        let new_philos = Philosopher::new("expected".into(), CoreSchool::Skeptic, 16);
        let result = player_hand.play_philosopher(Card::Philosopher(new_philos));
        assert!(result.is_ok());
        assert_eq!(
            player_hand.active_philosopher.unwrap().philosopher.name,
            "expected"
        );
        assert_ne!(player_hand.inactive_cards.len(), 0)
    }

    #[test]
    fn test_add_cards_to_empty() {
        let mut player_hand = PlayerHand {
            active_philosopher: None,
            inactive_cards: vec![],
            max_cards_in_hand: 7,
        };
        let new_cards = get_example_cards();
        let num_new_cards = new_cards.len();
        let result = player_hand.add_cards_to_hand(new_cards);
        assert!(result.is_ok());
        assert_eq!(player_hand.inactive_cards.len(), num_new_cards);
    }

    #[test]
    fn test_add_cards_to_existing_cards() {
        let mut player_hand = get_populated_player_hand(16);
        let num_existing_cards = player_hand.inactive_cards.len();
        let new_cards = get_example_cards();
        let num_new_cards = new_cards.len();
        let result = player_hand.add_cards_to_hand(new_cards);
        assert!(result.is_ok());
        assert_eq!(
            player_hand.inactive_cards.len(),
            num_new_cards + num_existing_cards
        )
    }

    #[test]
    fn test_num_available_slots_in_hand() {
        let player_hand = get_populated_player_hand(16);
        let num_existing_cards: u8 = player_hand.inactive_cards.len().try_into().unwrap();
        assert_eq!(
            player_hand.num_available_slots_in_hand(),
            8 - num_existing_cards
        );
    }

    #[test]
    fn test_num_available_slots_in_hand_no_slots() {
        let mut player_hand = get_populated_player_hand(16);
        let _ = player_hand.add_cards_to_hand(get_example_cards());
        let _ = player_hand.add_cards_to_hand(get_example_cards());
        let _ = player_hand.add_cards_to_hand(get_example_cards());
        let _ = player_hand.add_cards_to_hand(get_example_cards());
        assert_eq!(player_hand.num_available_slots_in_hand(), 0);
    }
}
