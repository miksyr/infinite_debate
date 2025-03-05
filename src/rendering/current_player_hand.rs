use ratatui::{
    style::{
        palette::tailwind::{GREEN, SLATE},
        Color,
    },
    text::Line,
    widgets::ListItem,
};

use crate::entities::Card;

pub struct AvailablePlayerCard<'a> {
    pub card: &'a Card,
    pub is_selected: CardSelectionState,
}

// how to convert AvailablePlayerCard into ListItem
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

#[derive(Copy, Clone, PartialEq)]
pub enum CardSelectionState {
    Selected,
    NotSelected,
}
