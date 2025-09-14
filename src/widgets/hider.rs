use crate::prelude::*;

/// Wraps around another widget to hide it.
/// 
/// [Widget::set_bounds] and [Widget::process_event] are passed through.
pub struct Hider<T> where T: Widget {
    pub inner: T,
    hidden: bool,
}

impl<T> Hider<T> where T: Widget {
    pub fn new(inner: T, hidden: bool) -> Self {
        Self { inner, hidden }
    }
    pub fn hidden(&self) -> bool {
        self.hidden
    }
    pub fn hide(&mut self) -> &mut Self {
        self.hidden = true;
        self
    }
    pub fn show(&mut self) -> &mut Self {
        self.hidden = false;
        self
    }
}

impl<T> Widget for Hider<T> where T: Widget {
    fn draw(&self, ctx: &mut DrawContext) {
        if !self.hidden {
            self.inner.draw(ctx);
        }
    }

    fn process_event(&mut self, e: &Event) -> bool {
        self.inner.process_event(e)
    }

    fn set_bounds(&mut self, bounds: Rect) {
        self.inner.set_bounds(bounds);
    }
}
