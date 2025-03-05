use ratatui::{
    style::{
        palette::tailwind::{GREEN, SLATE},
        Color,
    },
    text::Line,
    widgets::ListItem,
};

use crate::entities::Card;
use crate::player::PlayerHand;

pub struct AvailablePlayerCard<'a> {
    pub card: &'a Card,
    pub is_selected: CardSelectionState,
}

// how to convert Card into ListItem (for display)
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

impl<'a> From<&'a AvailablePlayerCard<'a>> for ListItem<'a> {
    fn from(value: &'a AvailablePlayerCard<'a>) -> Self {
        let line = match value.is_selected {
            CardSelectionState::NotSelected => {
                Line::styled(format!(" ☐ {:?}", value.card), TEXT_FG_COLOR)
            }
            CardSelectionState::Selected => {
                Line::styled(format!(" ✓ {:?}", value.card), COMPLETED_TEXT_FG_COLOR)
            }
        };
        ListItem::new(line)
    }
}

pub enum CardSelectionState {
    Selected,
    NotSelected,
}
impl CardSelectionState {
    fn toggle(&mut self) -> Self {
        match self {
            CardSelectionState::Selected => CardSelectionState::NotSelected,
            CardSelectionState::NotSelected => CardSelectionState::Selected,
        }
    }
}

pub struct CurrentPlayerHand<'a> {
    pub cards: Vec<AvailablePlayerCard<'a>>,
}
impl CurrentPlayerHand<'_> {
    fn convert_to_available_cards(cards: &Vec<Box<Card>>) -> Vec<AvailablePlayerCard> {
        let available_cards: Vec<AvailablePlayerCard> = cards
            .iter()
            .map(|card| AvailablePlayerCard {
                card: card.as_ref(),
                is_selected: CardSelectionState::NotSelected,
            })
            .collect();
        available_cards
    }
    pub fn from_player_hand(player_hand: &PlayerHand) -> CurrentPlayerHand {
        CurrentPlayerHand {
            cards: CurrentPlayerHand::convert_to_available_cards(&player_hand.inactive_cards),
        }
    }
    pub fn toggle_card_state(&mut self, card_index: usize) {
        if let Some(card) = self.cards.get_mut(card_index) {
            card.is_selected = card.is_selected.toggle();
        }
    }
}
