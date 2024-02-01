use std::{
    error::Error,
    io::{stderr, stdout, Write},
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    widgets::{canvas::Rectangle, Block, Borders, Padding},
    Terminal,
};

use rand::{rngs::OsRng, seq::SliceRandom};
use tui_textarea::{Input, Key, TextArea};


const DATA: &'static str = include_str!("br_utf8.txt");
fn get_words()->Vec<&'static str>{
    let mut words: Vec<&str> = DATA.split("\n").map(|f|f.trim()).collect();
    words.shuffle(&mut OsRng);
    words
}

fn main() -> Result<(), Box<(dyn Error)>> {
    terminal::enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Thick)
            .padding(Padding::horizontal(2)),
    );

    let layout = Layout::new(
        ratatui::layout::Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Length(3),
        ]
    ).split(terminal.size()?);
    loop {
        terminal.draw(|f| {
            f.render_widget(textarea.widget(), layout[1]);
        })?;

        match crossterm::event::read()?.into(){
            Input {key: Key::Char('m'), ctrl: true, alt: false, shift: false} | Input{key: Key::Enter, ..} => continue,
            Input {key: Key::Char(' '),..} => continue,
            Input {key: Key::Esc, ..} => break,
            input=>{
                textarea.input(input);
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(stderr(), LeaveAlternateScreen)?;

    Ok(())
}
