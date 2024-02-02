use std::{sync::mpsc::{channel, Receiver, Sender}, thread::{self, JoinHandle}, time::Duration};

use crossterm::event;
use tui_textarea::Input;

pub enum Event{
    Tick,
    IncreaseTime,
    Key(Input)
}

#[derive(Debug)]
pub struct EventHandler{
    receiver: Receiver<Event>,
    pub handler: (JoinHandle<()>, JoinHandle<()>)    
}

impl EventHandler {
    
    pub fn new()->Self{
        
        let (sender,receiver): (Sender<Event>, Receiver<Event>) = channel();
        
        let timer_handler = {
            let sender = sender.clone();
            thread::spawn(move||{
                loop {
                    sender.send(Event::IncreaseTime).expect("unable to send event increasetime");
                    sender.send(Event::Tick).expect("unable to send event tick");
                    thread::sleep(Duration::from_secs(1));
                }
            })
        };
        let key_handler = {
            let sender = sender.clone();
            thread::spawn(move||{
                loop {
                    sender.send(Event::Tick).expect("unable to send event tick");
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

        Self {receiver: receiver, handler: (timer_handler, key_handler) }
    }

    pub fn next(&self)->Result<Event, std::sync::mpsc::RecvError>{
        self.receiver.recv()
    }

}