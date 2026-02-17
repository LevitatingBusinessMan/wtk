pub mod button;
pub use button::Button;
pub mod label;
pub use label::Label;
pub mod wbox;
pub use wbox::WBox;
//pub mod radio_button;
//pub use radio_button::RadioButton;
pub mod entry;
pub use entry::Entry;
pub mod hider;
pub use hider::Hider;
pub mod centered;
pub use centered::Centered;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::event::Event;
use crate::prelude::*;

/// Base widget trait
pub trait Widget {
    fn draw(&self, ctx: &mut DrawContext);
    /// Pass en event to the widget. The bounds tells the widget its absolute location.
    fn process_event(&mut self, _e: &Event, _bounds: Rect) -> bool { false }
}

/// Widget with bounds tracking for use in widget containers
pub struct ChildWidget {
    pub widget: SharedWidget,
    pub bounds: Cell<Rect>,
}

impl ChildWidget {
    pub fn new(widget: SharedWidget) -> Self {
        Self { widget, bounds: Cell::new(Rect::zero()) }
    }
}

pub type SharedWidget = Rc<RefCell<dyn Widget>>;
//pub type Shared<T> = Rc<RefCell<T>>;

pub trait IntoShared<T> {
    fn shared(self) -> Rc<RefCell<T>>;
}

impl<T> IntoShared<T> for T {
    fn shared(self) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(self))
    }
}
