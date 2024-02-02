use std::error::Error;

use ratatui::{layout::{Constraint, Layout}, widgets::{Block, Borders, Padding, Paragraph, Wrap}};

use crate::{app::App, tui::CrosstermTerminal};



pub fn draw(app: &mut App, terminal: &mut CrosstermTerminal)->Result<(), Box<(dyn Error)>>{
    

    app.textarea.set_block(
        Block::new()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Thick)
            .padding(Padding::horizontal(2)),
    );

    let vertical_layout = Layout::new(
        ratatui::layout::Direction::Vertical,
        [
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3)
        ],
    )
    .split(terminal.size()?);

    let counter_layout = Layout::new(
        ratatui::layout::Direction::Horizontal,
        [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]
    ).split(vertical_layout[1]);

    
    terminal.draw(|f| {
            
        let paragraph_block = Block::new()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Thick);

        // render preview
        f.render_widget(Paragraph::new(vec![app.get_actual_words().into()]).wrap(Wrap{trim:true}).block(paragraph_block), vertical_layout[0]);
        
        // render word counter
        f.render_widget(Paragraph::new(format!("Elapsed time: {}",app.elapsed_seconds)).alignment(ratatui::layout::Alignment::Center), counter_layout[0]); // incorrect
        f.render_widget(Paragraph::new(format!("{} correct",app.correct_words)).alignment(ratatui::layout::Alignment::Center), counter_layout[1]); // correct
        f.render_widget(Paragraph::new(format!("{} incorrect",app.incorrect_words)).alignment(ratatui::layout::Alignment::Center), counter_layout[2]); // incorrect

        // render text field
        f.render_widget(app.textarea.widget(), vertical_layout[2]);
    })?;
    Ok(())
}