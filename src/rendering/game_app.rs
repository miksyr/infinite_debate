use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style},
    text::Line,
    widgets::{
        Block, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
    DefaultTerminal,
};

use crate::entities::Card;
use crate::game_management::GameBoard;
use crate::rendering::{AvailablePlayerCard, CardSelectionState};

const LIST_HIGHLIGHT_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct GameApp {
    exit: bool,
    game_board: GameBoard,
    current_round: u32,
    current_card_state: ListState,
    selected_cards: Vec<CardSelectionState>,
}
impl GameApp {
    pub fn new() -> Self {
        let game_board = GameBoard::new(None);
        let num_cards = game_board.game_config.max_cards_in_hand();
        GameApp {
            exit: false,
            game_board,
            current_round: 0,
            current_card_state: ListState::default(),
            selected_cards: vec![CardSelectionState::NotSelected; num_cards.into()],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Up | KeyCode::Char('w') => self.select_previous(),
            KeyCode::Down | KeyCode::Char('s') => self.select_next(),
            KeyCode::Right | KeyCode::Char('d') => self
                .toggle_card_selection()
                .expect("Couldn't toggle card selection"),
            KeyCode::Enter => self.submit_card_selections(),
            _ => {}
        }
    }

    fn select_previous(&mut self) {
        self.current_card_state.select_previous();
    }

    fn select_next(&mut self) {
        self.current_card_state.select_next();
    }

    fn toggle_card_selection(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(i) = self.current_card_state.selected() {
            let selected_count = self
                .selected_cards
                .iter()
                .filter(|&&s| s == CardSelectionState::Selected)
                .count();

            if self.selected_cards[i] == CardSelectionState::Selected {
                self.selected_cards[i] = CardSelectionState::NotSelected;
            } else if selected_count < 3 {
                self.selected_cards[i] = CardSelectionState::Selected;
            }
        }
        Ok(())
    }

    fn reset_card_selection_state(&mut self) {
        let num_cards = self.game_board.game_config.max_cards_in_hand();
        self.selected_cards = vec![CardSelectionState::NotSelected; num_cards.into()];
    }

    fn submit_card_selections(&mut self) {
        let (active_hand, _active_deck) = self
            .game_board
            .active_player_data()
            .expect("can't get active player data");

        let selected_cards: Vec<&Card> = active_hand
            .inactive_cards
            .iter()
            .enumerate()
            .filter_map(|(i, card)| {
                if self.selected_cards.get(i) == Some(&CardSelectionState::Selected) {
                    Some(card.as_ref())
                } else {
                    None
                }
            })
            .collect();

        if !selected_cards.is_empty() {
            let owned_cards: Vec<Card> = selected_cards.into_iter().cloned().collect();
            self.game_board
                .process_turn(owned_cards)
                .expect("couldn't process turn");
        }
        self.reset_card_selection_state();
    }
}

// actual rendering logic
impl GameApp {
    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            format!("Round: {} --- Use ↓↑ to move, ← to unselect all, → to add/remove card, [Enter] to end turn", self.current_round),
        )
        .centered()
        .render(area, buf);
    }

    fn render_opponent_philosophers(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Opponent Philosophers").centered());
        let (inactive_hand, _inactive_deck) = self
            .game_board
            .inactive_player_data()
            .expect("can't get inactive player data");
        Paragraph::new(format!("{:?}", inactive_hand.active_philosopher))
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_player_philosophers(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Player Philosophers").centered());
        let (active_hand, _active_deck) = self
            .game_board
            .active_player_data()
            .expect("can't get active player data");
        Paragraph::new(format!("{:?}", active_hand.active_philosopher))
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_available_cards(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered().title(Line::raw("Player Available Cards").centered());
        let (active_hand, _active_deck) = self
            .game_board
            .active_player_data()
            .expect("can't get active player data");
        let available_cards: Vec<AvailablePlayerCard> = active_hand
            .inactive_cards
            .iter()
            .enumerate()
            .map(|(i, card)| AvailablePlayerCard {
                card: card.as_ref(),
                is_selected: self
                    .selected_cards
                    .get(i)
                    .copied()
                    .unwrap_or(CardSelectionState::NotSelected),
            })
            .collect();

        let list_items: Vec<ListItem> = available_cards.iter().map(ListItem::from).collect();

        let list = List::new(list_items)
            .block(block)
            .highlight_style(LIST_HIGHLIGHT_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.current_card_state);
    }
}
impl Widget for &mut GameApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [game_board_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let [opponent_philosopher, player_philosopher, player_available_cards] =
            Layout::vertical([
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 2),
            ])
            .areas(game_board_area);

        self.render_footer(footer_area, buf);
        self.render_opponent_philosophers(opponent_philosopher, buf);
        self.render_player_philosophers(player_philosopher, buf);
        self.render_available_cards(player_available_cards, buf);
    }
}
