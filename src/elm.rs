//! A collection of helper traits and macros for creating an Elm Architecture like app.
use std::cell::RefCell;
use std::rc::Rc;

use std::sync::mpsc;
use crate::prelude::*;

pub trait ElmModel {
    type Message;
    /// Send a message to the model
    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message>;
    /// Process all events. Returns true if a draw is required.
    fn update_all(&mut self) -> bool {
        let mut draw = false;
        loop {
            match self.receiver().try_recv() {
                Ok(msg) => {
                    draw = self.update(msg);
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
    fn update(&mut self, msg: Self::Message) -> bool;
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
            let draw = model.borrow_mut().update_all() || draw;
            if draw { self.draw(); }
        }
    }
}
