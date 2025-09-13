//! A collection of helper traits and macros for creating an Elm Architecture like app.
//! 
//! Within the [Elm Architecture](https://guide.elm-lang.org/architecture/) the state of your application is contained in a **model**.
//! UI interactions cause messages to be sent to the model, which **updates** its state accordingly.
//! The model is then used to draw the **view** of the application.
//! 
//! Wtk is flexible enough that this design can be achieved quite easily.
//! Typically you would use a single top-level widget which acts as the model.
//! You then configure all the child widgets with callbacks which send messages to the model.
//! 
//! The [crate::elm] module contains a few traits and macros which can simplify this design.
//! If [ElmModel] is implemented for your top-level widget, you can use [App::elm_run] to run your application.
//! [App::elm_run] is similar to [App::run] except that it calls [ElmModel::update_all] on your model after each event.
//! 
//! For creating closures, the [crate::elm_cb] macro may be used.
//! For example:
//! 
//! ```
//! Button::new("+", elm_cb!(sender, _b => CounterMessage::Increment)).shared();
//! ```
//! 
//! For an example app which used the Elm Architecture see the elm_counter en media_player examples.
use std::cell::RefCell;
use std::rc::Rc;

use std::sync::mpsc;
use crate::prelude::*;

/**
 * A helper trait for simplifying an elm architecture design.
 * 
 * An example:
 * ```
 * pub struct Counter {
 *     count: i32,
 *     button_box: Rc<RefCell<WBox>>,
 *     counter_label: Rc<RefCell<Label>>,
 *     receiver: mpsc::Receiver<CounterMessage>,
 * }
 * 
 * impl ElmModel for Counter {
 *     type Message = CounterMessage;
 * 
 *     fn receiver(&mut self) -> &mut std::sync::mpsc::Receiver<Self::Message> {
 *         &mut self.receiver
 *     }
 * 
 *     fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) where B: Backend {
 *         match msg {
 *             CounterMessage::Increment => self.count += 1,
 *             CounterMessage::Decrement => self.count -= 1,
 *         }
 *         self.counter_label.borrow_mut().set_text(format!("Count: {}", self.count));
 *     }
 * }
 * ```
 */
pub trait ElmModel{
    type Message;
    /// Send a message to the model
    fn receiver(&mut self) -> &mut mpsc::Receiver<Self::Message>;

    /// Process all messages. Returns true if any messages were processed (implying a draw).
    fn update_all<B>(&mut self, app: &mut App<B>) -> bool where B: Backend  {
        let mut draw = false;
        loop {
            match self.receiver().try_recv() {
                Ok(msg) => {
                    self.update(app, msg);
                    draw = true;
                },
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => break,
                    mpsc::TryRecvError::Disconnected => panic!(),
                },
            }
        }
        draw
    }
    /// Process a single message.
    fn update<B>(&mut self, app: &mut App<B>, msg: Self::Message) where B: Backend;
}


/// A shorthand for creating a closure that sends a message over a reference counted channel.
/// 
/// The first argument should be something like `Rc<Sender>`. The other arguments should be arguments to be received by the closure.
/// These arguments are followed by a `=>` and an expression resulting in the message to be sent.
/// 
/// For example:
/// ```
/// Button::new("+", elm_cb!(sender, _b => CounterMessage::Increment)).shared();
/// ```
/// 
/// Don't forget to use [enclose], which is used by this macro.
#[macro_export]
macro_rules! elm_cb {
    ($sender:ident, $($param:pat),+ => $message:expr) => {
        enclose!(($sender) move |$($param),+| $sender.send($message).unwrap())
    };
    // Multiple identifiers case with explicit sender
    (($($ident:ident),+) $sender:ident, $($param:pat),+ => $message:expr) => {
        enclose!(($($ident),+) move |$($param),+| $sender.send($message).unwrap())
    };
}

/**
 * Adds the [App::elm_run] method to [App].
 */
pub trait ElmLoop {
    /// Like [App::run] but involves updating an elm model.
    /// 
    /// Don't forget that if your model is also your widget, the widget still needs to be added to the app to be drawn.
    /// 
    /// Example:
    /// ```
    /// fn main() {
    ///     let mut app = App::<SDLBackend>::new("Counter go brrr");
    ///     let counter = Counter::new().shared();
    ///     app.add_widget(counter.clone());
    ///     app.elm_run(counter);    
    /// }
    /// ```
    fn elm_run<M>(&mut self, model: Rc<RefCell<M>>) where M: ElmModel;
}

impl<B> ElmLoop for App<B> where B: Backend {
    fn elm_run<M>(&mut self, model: Rc<RefCell<M>>) where M: ElmModel {
        self.draw();
        while !self.quit {
            let draw = self.poll_and_process_event();
            let draw = model.borrow_mut().update_all(self) || draw;
            if draw { self.draw(); }
        }
    }
}
