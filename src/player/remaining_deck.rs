use crate::entities::Card;
use rand::{rng, RngCore};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

#[derive(Debug)]
pub struct RemainingDeck {
    cards: Vec<Box<Card>>,
}
impl RemainingDeck {
    pub fn new(mut cards: Vec<Box<Card>>, seed: Option<u64>) -> Self {
        let mut rng: Box<dyn RngCore> = match seed {
            Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
            None => Box::new(rng()),
        };
        cards.shuffle(&mut rng);
        RemainingDeck { cards }
    }
    pub fn num_remaining_cards(&self) -> u8 {
        self.cards.len().try_into().unwrap()
    }
    pub fn draw_new_cards(&mut self, n: u8) -> Result<Vec<Box<Card>>, Box<dyn std::error::Error>> {
        let n: u8 = n.min(self.num_remaining_cards());
        let selected_cards: Vec<Box<Card>> = self.cards.drain(0..n as usize).collect();
        Ok(selected_cards)
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils::get_example_cards;

    use super::*;

    #[test]
    fn test_new_remaining_deck_with_seed() {
        let cards = get_example_cards();
        let remaining_deck = RemainingDeck::new(cards, Some(42));
        assert_ne!(
            format!("{:?}", get_example_cards()),
            format!("{:?}", remaining_deck.cards)
        );
        assert_ne!(remaining_deck.num_remaining_cards(), 0);
    }

    #[test]
    fn test_new_remaining_deck_without_seed() {
        let cards = get_example_cards();
        let remaining_deck = RemainingDeck::new(cards, None);
        assert_ne!(remaining_deck.num_remaining_cards(), 0);
    }

    #[test]
    fn test_draw_new_cards_with_cards_left() {
        let cards = get_example_cards();
        let num_original_cards: u8 = cards.len().try_into().unwrap();
        let mut remaining_deck = RemainingDeck::new(cards, None);
        assert_eq!(remaining_deck.num_remaining_cards(), num_original_cards);
        let drawn_cards = remaining_deck
            .draw_new_cards(2)
            .expect("should have been able to draw cards");
        assert_eq!(drawn_cards.len(), 2);
        assert_eq!(remaining_deck.num_remaining_cards(), 1);
    }

    #[test]
    fn test_draw_new_cards_with_none_left() {
        let mut remaining_deck = RemainingDeck::new(vec![], None);
        let drawn_cards = remaining_deck
            .draw_new_cards(64)
            .expect("should have been able to draw cards");
        assert!(drawn_cards.len() < 64);
        assert_eq!(remaining_deck.num_remaining_cards(), 0);
    }

    #[test]
    fn test_draw_new_cards_with_not_enough_left() {
        let cards = get_example_cards();
        let num_original_cards: u8 = cards.len().try_into().unwrap();
        let mut remaining_deck = RemainingDeck::new(cards, None);
        assert_eq!(remaining_deck.num_remaining_cards(), num_original_cards);
        let drawn_cards = remaining_deck
            .draw_new_cards(64)
            .expect("should have been able to draw cards");
        assert!(drawn_cards.len() < 64);
        assert_eq!(remaining_deck.num_remaining_cards(), 0);
    }
}
