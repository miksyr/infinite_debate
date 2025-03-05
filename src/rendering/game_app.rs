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

use crate::game_management::GameBoard;
use crate::rendering::{AvailablePlayerCard, CardSelectionState, CurrentPlayerHand};

const LIST_HIGHLIGHT_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct GameApp {
    exit: bool,
    game_board: GameBoard,
    current_round: u32,
}
impl GameApp {
    pub fn new() -> Self {
        let mut game_board = GameBoard::new(None);
        let current_player_data = game_board.active_player_data().unwrap();
        let current_player_hand = CurrentPlayerHand::from_player_hand(current_player_data.0);
        GameApp {
            exit: false,
            game_board,
            current_round: 0,
        }
    }

    fn get_current_player_hand(&mut self) -> Result<CurrentPlayerHand, Box<dyn std::error::Error>> {
        let current_player_data = self.game_board.active_player_data().unwrap();
        let current_player_hand = CurrentPlayerHand::from_player_hand(current_player_data.0);
        Ok(current_player_hand)
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
            KeyCode::Enter => self.submit_card_selections(),
            _ => {}
        }
    }

    fn select_previous(&mut self) {
        // self.current_player_hand.card_state.select_previous();
    }

    fn select_next(&mut self) {
        // self.current_player_hand.card_state.select_next();
    }

    fn toggle_card_selection(&mut self) {
        todo!()
        // if let Some(i) = self.current_card_state.selected() {
        //     self.todo_list.items[i].status = match self.todo_list.items[i].status {
        //         CardSelectionState::Selected => CardSelectionState::NotSelected,
        //         CardSelectionState::NotSelected => CardSelectionState::Selected,
        //     }
        // }
    }

    fn submit_card_selections(&mut self) {
        todo!()
    }
}

// App rendering logic
impl GameApp {
    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "Use ↓↑ to move, ← to unselect all, → to add/remove card, [Enter] to end turn",
        )
        .centered()
        .render(area, buf);
    }

    fn render_opponent_philosophers(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Opponent Philosophers").centered());
        Paragraph::new(format!(
            "{:?}",
            self.game_board.player_2_hand.active_philosopher
        ))
        .centered()
        .block(block)
        .render(area, buf);
    }

    fn render_player_philosophers(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Player Philosophers").centered());
        Paragraph::new(format!(
            "{:?}",
            self.game_board.player_1_hand.active_philosopher
        ))
        .centered()
        .block(block)
        .render(area, buf);
    }

    fn render_available_cards(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Player Available Cards").centered());
        let available_cards: Vec<AvailablePlayerCard> = self
            .game_board
            .player_1_hand
            .inactive_cards
            .iter()
            .map(|card| AvailablePlayerCard {
                card: card.as_ref(),
                is_selected: CardSelectionState::NotSelected,
            })
            .collect();
        let list_items: Vec<ListItem> = available_cards
            .iter()
            .map(|card| ListItem::from(card))
            .collect();
        let list = List::new(list_items)
            .block(block)
            .highlight_style(LIST_HIGHLIGHT_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut ListState::default());
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

        GameApp::render_footer(footer_area, buf);
        self.render_opponent_philosophers(opponent_philosopher, buf);
        self.render_player_philosophers(player_philosopher, buf);
        self.render_available_cards(player_available_cards, buf);
    }
}
