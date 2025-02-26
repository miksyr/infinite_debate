use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{GREEN, SLATE},
        Color, Modifier, Style,
    },
    text::Line,
    widgets::{
        Block, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget, Widget,
    },
    DefaultTerminal,
};

use super::{entities, game_management::GameBoard};

pub struct GameApp {
    exit: bool,
    game_board: GameBoard,
    current_round: u32,
}

// how to convert Card into ListItem (for display)
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl<'a> From<&'a AvailablePlayerCard<'a>> for ListItem<'a> {
    fn from(value: &'a AvailablePlayerCard<'a>) -> Self {
        let line = match value.is_selected {
            CardSelectionState::NotSelected => {
                Line::styled(format!(" ☐ {}", value.card.get_name()), TEXT_FG_COLOR)
            }
            CardSelectionState::Selected => Line::styled(
                format!(" ✓ {}", value.card.get_name()),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}

enum CardSelectionState {
    Selected,
    NotSelected,
}

struct AvailablePlayerCard<'a> {
    card: &'a dyn entities::Card,
    is_selected: CardSelectionState,
}

impl GameApp {
    pub fn new() -> Self {
        GameApp {
            exit: false,
            game_board: GameBoard::new(),
            current_round: 0,
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
            _ => {}
        }
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
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // this list state should contain whether a card is selected or not
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
