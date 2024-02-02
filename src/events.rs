use std::{sync::mpsc::{channel, Receiver, Sender}, thread::{self, JoinHandle}, time::Duration};

use crossterm::event;
use tui_textarea::Input;

pub enum Event{
    Tick,
    Key(Input)
}

#[derive(Debug)]
pub struct EventHandler{
    sender: Sender<Event>,
    receiver: Receiver<Event>,
    handler: JoinHandle<()>    
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
                                sender.send(Event::Key(e.into()));
                           },
                           _=>{
                                sender.send(Event::Tick);
                           } 
                        }
                    }
                }
            })
        };

        Self { sender: sender, receiver: receiver, handler: handler }
    }

    pub fn next(&self)->Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }

}