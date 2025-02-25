use std::time::{Duration, Instant};

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    widgets::{ListState, Widget},
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
        let tick_rate = Duration::from_millis(16);
        while !self.exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {}

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

impl Widget for GameApp {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}

