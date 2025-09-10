pub mod button;
pub use button::Button;

use std::cell::RefCell;
use std::rc::Rc;

use crate::event::Event;
use crate::prelude::*;

/// Base widget trait
pub trait Widget {
    fn draw<B>(&self, ctx: &mut DrawContext<B>) where B: DrawBackend;
    fn process_event(&mut self, event: &Event);
    fn size(&self) -> (u32,u32);
}

pub type SharedWidget = Rc<RefCell<dyn Widget>>;

pub trait IntoShared<T> {
    fn shared(self) -> Rc<RefCell<T>>;
}

impl<T> IntoShared<T> for T {
    fn shared(self) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(self))
    }
}
