use rand::{rngs::OsRng, seq::SliceRandom};
use ratatui::{style::Stylize, text::{Line, Span}};
use tui_textarea::TextArea;

const DATA: &'static str = include_str!("br_utf8.txt");


#[derive(Debug,Default)]
pub struct App<'a>{
    pub correct_words: u16,
    pub incorrect_words: u16,
    pub elapsed_seconds: u16,
    pub index: usize,
    pub should_quit: bool,
    pub summary: Option<String>, // 0 means user exited, 1 means user finished test and got summary
    pub textarea: TextArea<'a>,
    words: Vec<&'a str>,
}

impl<'a> App<'a> { 

    pub fn quit(&mut self){
        self.should_quit = true;
    }

    pub fn get_user_input(&self) -> String{
        self.textarea.lines().first().unwrap().to_string()
    }

    pub fn get_current_word(&self ) -> String{
        self.words[self.index].to_string()
    }

    pub fn matched_text_colorizer(&self)->Vec<Span>{
        let current_word = self.get_current_word();
        let input = self.get_user_input();

        let mut result: Vec<Span> = Vec::new();

        for (index, letter) in current_word.chars().enumerate(){
            if let Some(res) = input.chars().nth(index){
                if letter.eq(&res){
                    result.push(Span::raw(letter.to_string()).bold().green());
                }else{
                    result.push(Span::raw(letter.to_string()).bold().red());
                }
            }else{
                result.push(Span::raw(letter.to_string()).bold());
            }
        }
        result.push(Span::raw(" "));
        result
    }

    pub fn get_actual_words(&self)->Line{
        let max_displaying_words = 50;

        let mut words_span: Vec<Span> = Vec::new();
        words_span.append(&mut self.matched_text_colorizer());
        words_span.append(&mut self.words[self.index+1..max_displaying_words+self.index]
            .iter()
            .map(|f|Span::raw(format!("{} ", f)))
            .collect::<Vec<Span>>());

        words_span.into()
    }
    fn get_words() -> Vec<&'static str> {
        let mut words: Vec<&str> = DATA.split("\n").map(|f| f.trim()).collect();
        words.shuffle(&mut OsRng);
        words
    }
    pub fn new()->Self{
        Self { correct_words: 0, incorrect_words: 0, index: 0 , elapsed_seconds: 0, summary: None, textarea: TextArea::default(),  words: App::get_words(), should_quit: false }
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

    pub fn increase_elapsed_time(&mut self){
        if let Some(res) = self.elapsed_seconds.checked_add(1){
            self.elapsed_seconds = res;
        }
    }

    pub fn clear_current_input(&mut self){
        self.textarea.delete_line_by_head();
    }

    pub fn get_summary(&mut self){
        self.quit();
        
        let words_per_minute = (60.0/self.elapsed_seconds as f32)*(self.correct_words+self.incorrect_words)as f32;

        let correct_words_percentage = (self.correct_words as f32 / (self.correct_words+self.incorrect_words) as f32) * 100.0;
        let incorrect_words_percentage = (self.incorrect_words as f32 / (self.correct_words+self.incorrect_words) as f32) * 100.0;

        self.summary = Some(format!("Summary:\n\nElapsed time: {}s\nCorrect words: {}\tIncorrect words: {}\n\n{} words per minute!\n{:.2}% correct\t{:.2}% incorrect",
            self.elapsed_seconds,
            self.correct_words,
            self.incorrect_words,
            words_per_minute.round(),
            correct_words_percentage,
            incorrect_words_percentage
        ));                

    }
}