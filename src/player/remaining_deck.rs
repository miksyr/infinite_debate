use crate::entities::Card;
use rand::prelude::SliceRandom;
use rand::rng;

#[derive(Debug)]
pub struct RemainingDeck {
    cards: Vec<Box<Card>>,
}
impl RemainingDeck {
    pub fn new(mut cards: Vec<Box<Card>>) -> Self {
        cards.shuffle(&mut rng());
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
    #[test]
    fn test() {}
}
