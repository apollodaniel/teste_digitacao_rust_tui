use std::{error::Error, io::stderr, panic};

use crossterm::{execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::events::EventHandler;

pub type CrosstermTerminal = Terminal<CrosstermBackend<std::io::Stderr>>;

#[derive(Debug)]
pub struct Tui{
    pub terminal: CrosstermTerminal,
    pub events: EventHandler
}

impl Tui {

    pub fn new(terminal: CrosstermTerminal, events: EventHandler)->Self{
        Self { terminal: terminal, events: events }
    }
    pub fn enter()->Result<(), Box<(dyn Error )>>{
        terminal::enable_raw_mode()?;
        execute!(stderr(), EnterAlternateScreen)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move|panic|{
            Self::reset().expect("unable to reset tui");
            panic_hook(panic);
        }));

        Ok(())
    }

    pub fn reset()->Result<(), Box<(dyn Error )>>{
        terminal::disable_raw_mode()?;
        execute!(stderr(), LeaveAlternateScreen)?;
        Ok(())
    }

}