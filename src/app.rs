use rand::{rngs::OsRng, seq::SliceRandom};
use ratatui::{style::Stylize, text::Span};
use tui_textarea::TextArea;

const DATA: &'static str = include_str!("br_utf8.txt");


#[derive(Debug,Default)]
pub struct App<'a>{
    pub correct_words: u16,
    pub incorrect_words: u16,
    pub index: usize,
    pub should_quit: bool,
    pub textarea: TextArea<'a>,
    words: Vec<&'a str>,
}

impl<'a> App<'a> {
    pub fn get_user_input(&self) -> String{
        self.textarea.lines().first().unwrap().to_string()
    }

    pub fn get_current_word(&self ) -> String{
        self.words[self.index].to_string()
    }
    
    pub fn get_actual_words(&self)->Vec<Span>{
        let max_displaying_words = 50;
        self.words[self.index..max_displaying_words+self.index]
        .into_iter()
        .map(|f| {
            let span = Span::raw(format!("{} ", f));
            if f.eq(&self.words[self.index]){
                if self.get_user_input().eq(""){
                    span.bold()
                }else if f.starts_with(self.get_user_input().as_str()) {
                    span.bold().green()
                }else{
                    span.bold().red()
                }
            }else{
                span
            }        
        })
        .collect()
    }
    fn get_words() -> Vec<&'static str> {
        let mut words: Vec<&str> = DATA.split("\n").map(|f| f.trim()).collect();
        words.shuffle(&mut OsRng);
        words
    }
    pub fn new()->Self{
        Self { correct_words: 0, incorrect_words: 0, index: 0 , textarea: TextArea::default(),  words: App::get_words(), should_quit: false }
    }

    pub fn increase_correct_words(&mut self){
        if let Some(res) = self.correct_words.checked_add(1) {
            self.correct_words=res;
        }
        self.index+=1;
    }
    pub fn increase_incorrect_words(&mut self){
        if let Some(res) = self.incorrect_words.checked_add(1) {
            self.incorrect_words=res;
        }
        self.index+=1;
    }

    pub fn clear_current_input(&mut self){
        self.textarea.delete_line_by_head();
    }
}