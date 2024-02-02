use std::{sync::mpsc::{channel, Receiver, Sender}, thread::{self, JoinHandle}};

use crossterm::event;
use tui_textarea::Input;

pub enum Event{
    Key(Input)
}

#[derive(Debug)]
pub struct EventHandler{
    receiver: Receiver<Event>,
    pub handler: JoinHandle<()>    
}

impl EventHandler {
    
    pub fn new()->Self{
        
        let (sender,receiver): (Sender<Event>, Receiver<Event>) = channel();
        
        let handler = {
            let sender = sender.clone();
            thread::spawn(move||{
                loop {
                    if let Ok(event) = event::read() {
                        match event {
                           event::Event::Key(e) => {
                                sender.send(Event::Key(e.into())).expect("Error sending key");
                           },
                           _=>{} 
                        }
                    }
                }
            })
        };

        Self {receiver: receiver, handler: handler }
    }

    pub fn next(&self)->Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }

}