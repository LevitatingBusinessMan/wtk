pub mod button;
pub use button::Button;
pub mod label;
pub use label::Label;

use std::cell::RefCell;
use std::rc::Rc;

use crate::event::Event;
use crate::prelude::*;

/// Base widget trait
pub trait Widget {
    fn draw(&self, ctx: &mut DrawContext);
    fn process_event(&mut self, event: &Event) -> bool { false }
    fn set_bounds(&mut self, bounds: Rect) {  }
    //fn as_any(&self) -> &dyn Any;
    //fn as_any_mut(&mut self) -> &mut dyn Any;
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
