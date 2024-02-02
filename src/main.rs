use std::{
    error::Error, io::{stderr, stdout, Write}, rc::Rc
};

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::EventHandler;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::{self, Line, Span},
    widgets::{canvas::Rectangle, Block, Borders, Padding, Paragraph, Wrap},
    Terminal,
};

use rand::{rngs::OsRng, seq::SliceRandom};
use tui::{CrosstermTerminal, Tui};
use tui_textarea::{Input, Key, TextArea};
use ui::draw;


pub mod app;
pub mod tui;
pub mod events;
pub mod ui;




fn main() -> Result<(), Box<(dyn Error)>> {

    let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    let mut app = App::new();

    let mut tui = Tui::new(terminal, EventHandler::new());

    Tui::enter()?;

    while !app.should_quit {
        draw(&mut app, &mut tui.terminal)?;
        if let Ok(event) = tui.events.next() {
            match event {
                events::Event::Tick=>{}
                events::Event::Key(key) =>{
                    match key {
                        Input {
                            key: Key::Char('m'),
                            ctrl: true,
                            alt: false,
                            shift: false,
                        }
                        | Input {
                            key: Key::Enter, ..
                        } => continue,
                        Input {
                            key: Key::Char(' '),
                            ..
                        } => {
                            if app.get_user_input().eq(&app.get_current_word()){
                                app.increase_correct_words();
                            }else{
                                app.increase_incorrect_words();
                            }
                            app.clear_current_input();
                            continue
                        },
                        Input { key: Key::Esc, .. } => break,
                        input => {
                            app.textarea.input(input);
                        }
                    }
                },
            }
        }  
    }
    Tui::reset()?;


    Ok(())
}
