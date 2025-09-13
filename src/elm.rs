//! A collection of helper traits and macros for creating an Elm Architecture like app.
//! 
//! For an example app which used the Elm Architecture see media_player example.
use std::cell::RefCell;
use std::rc::Rc;

use std::sync::mpsc;
use crate::prelude::*;

pub trait ElmModel{
    type Message;
    /// Send a message to the model
    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message>;
    /// Process all events. Returns true if a draw is required.
    fn update_all<B>(&mut self, app: &mut App<B>) -> bool where B: Backend  {
        let mut draw = false;
        loop {
            match self.receiver().try_recv() {
                Ok(msg) => {
                    draw = self.update(app, msg);
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => break,
                    mpsc::TryRecvError::Disconnected => panic!(),
                },
            }
        }
        draw
    }
    /// Process a single event. Returns true if a draw is required.
    fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) -> bool where B: Backend;
}


/// A shorthand for creating a closure that sends a message over a reference counted channel
#[macro_export]
macro_rules! elm_cb {
    ($sender:ident, $($param:pat),+ => $message:expr) => {
        enclose!(($sender) move |$($param),+| $sender.send($message).unwrap())
    };
}

pub trait EmlLoop {
    /// Like [App::run] but involves updating an elm model.
    fn elm_run<M>(&mut self, model: Rc<RefCell<M>>) where M: ElmModel;
}

impl<B> EmlLoop for App<B> where B: Backend {
    fn elm_run<M>(&mut self, model: Rc<RefCell<M>>) where M: ElmModel {
        self.draw();
        while !self.quit {
            let draw = self.poll_and_process_event();
            let draw = model.borrow_mut().update_all(self) || draw;
            if draw { self.draw(); }
        }
    }
}
