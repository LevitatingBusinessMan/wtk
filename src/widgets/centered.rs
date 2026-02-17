use crate::{draw::DrawContextInternal, prelude::*, widgets::ChildWidget};

/// Wraps around another widget to hide it.
/// 
/// [Widget::set_bounds] and [Widget::process_event] are passed through.
pub struct Centered {
    pub inner: ChildWidget,
    orientation: Orientation,
    length: usize,
}

impl Centered {
    pub fn new(inner: SharedWidget, orientation: Orientation, length: usize) -> Self {
        Self { inner: ChildWidget::new(inner), orientation, length }
    }
}

impl Widget for Centered {
    fn draw(&self, ctx: &mut DrawContext) {
        let mut child_ctx = DrawContext::new(ctx.zero_point());
        self.inner.widget.borrow().draw(&mut child_ctx);
        let child_bounds = child_ctx.bounds();
        
        let bounds_length = match self.orientation {
            Orientation::Horizontal => child_bounds.width,
            Orientation::Vertical => child_bounds.height,
        } as usize;
        
        if bounds_length < self.length {
            let add = (self.length - bounds_length) / 2;
            let offset = match self.orientation {
                Orientation::Horizontal => Point::new(add as u32, 0),
                Orientation::Vertical => Point::new(0, add as u32),
            };
            child_ctx.set_zero_point(ctx.zero_point() + offset);
            self.inner.bounds.set(offset.with_size(child_bounds.size()));
        }
        
        ctx.claim(match self.orientation {
            Orientation::Horizontal => Rect::new(0, 0, self.length as u32, child_bounds.height),
            Orientation::Vertical => Rect::new(0, 0, child_bounds.width, self.length as u32),
        });
        ctx.merge(child_ctx);
    }

    fn process_event(&mut self, e: &Event, bounds: Rect) -> bool {
        self.inner.widget.borrow_mut().process_event(e, bounds.point() + self.inner.bounds.get())
    }
}
