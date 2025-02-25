use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, ListState, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use super::{entities, game_management::GameBoard};

pub struct GameApp {
    exit: bool,
    game_board: GameBoard,
    round: u32,
}

struct AvailablePlayerCards {
    cards: Box<dyn entities::Card>,
    state: ListState,
}

impl GameApp {
    pub fn new() -> Self {
        GameApp {
            exit: false,
            game_board: GameBoard::new(),
            round: 0,
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
        Paragraph::new("Put stuff here")
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_player_philosophers(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Player Philosophers").centered());
        Paragraph::new("Put stuff here")
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_available_cards(&mut self, area: Rect, buf: &mut Buffer) {
        //  TODO: put actual logic here
        let block = Block::bordered().title(Line::raw("Player Available Cards").centered());
        Paragraph::new("Put stuff here")
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl Widget for &mut GameApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [game_board_area, footer_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(area);

        let [opponent_philosophers, player_philosophers, player_available_cards] =
            Layout::vertical([
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 2),
            ])
            .areas(game_board_area);

        GameApp::render_footer(footer_area, buf);
        self.render_opponent_philosophers(opponent_philosophers, buf);
        self.render_player_philosophers(player_philosophers, buf);
        self.render_available_cards(player_available_cards, buf);
    }
}
