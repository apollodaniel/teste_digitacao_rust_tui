use std::{
    error::Error, io::stderr
};

use app::App;
use events::EventHandler;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use tui::Tui;
use tui_textarea::{Input, Key};
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
        
        if let Ok(event) = tui.events.next() {
            match event {
                events::Event::Tick=> {draw(&mut app, &mut tui.terminal)?},
                events::Event::IncreaseTime=>{
                    app.increase_elapsed_time();
                }
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
                        Input { key: Key::Esc, .. } => app.quit(),
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
