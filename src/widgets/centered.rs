use crate::{draw::DrawContextInternal, prelude::*, widgets::ChildWidget};

/// Wraps around another widget to hide it.
/// 
/// [Widget::set_bounds] and [Widget::process_event] are passed through.
pub struct Centered {
    pub inner: ChildWidget,
    orientation: Orientation,
    length: u32,
}

impl Centered {
    pub fn new(inner: SharedWidget, orientation: Orientation, length: u32) -> Self {
        Self { inner: ChildWidget::new(inner), orientation, length }
    }
    pub fn set_length(&mut self, length: u32) {
        self.length = length;
    }
}

impl Widget for Centered {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut child_ctx = DrawContext::new();
        self.inner.widget.borrow().draw(&mut child_ctx);
        let child_size = child_ctx.size();

        let child_length = match self.orientation {
            Orientation::Horizontal => child_size.width,
            Orientation::Vertical => child_size.height,
        };

        let offset = if child_length < self.length {
            let add = (self.length - child_length) / 2;
            match self.orientation {
                Orientation::Horizontal => Point::new(add as u32, 0),
                Orientation::Vertical => Point::new(0, add as u32),
            }
        } else {
            Point::zero()
        };

        self.inner.bounds.set(offset.with_size(child_size));

        ctx.claim(match self.orientation {
            Orientation::Horizontal => Rect::new(0, 0, self.length as u32, child_size.height),
            Orientation::Vertical => Rect::new(0, 0, child_size.width, self.length as u32),
        });
        ctx.merge_at(offset, child_ctx);
    }

    fn process_event(&mut self, e: &Event, bounds: Rect) -> bool {
        self.inner.widget.borrow_mut().process_event(e, bounds.point() + self.inner.bounds.get())
    }
}
