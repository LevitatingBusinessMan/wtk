use std::cell::Cell;

use crate::prelude::*;
use super::SharedWidget;
use crate::draw;

/// Widget Box for grouping widgets
pub struct WBox {
    widgets: Vec<(SharedWidget, Cell<Rect>)>,    orientation: Orientation,
    padding: u32,
    margin: u32,
    border: bool,
}

impl WBox {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            widgets: vec![],
            orientation,
            border: false,
            padding: draw::DEFAULT_PADDING,
            margin: 0,
        }
    }
    pub fn with(orientation: Orientation, widgets: Vec<SharedWidget>) -> Self {
        Self {
            widgets: widgets.into_iter().map(|w| (w, Cell::new(Rect::zero()))).collect(),
            orientation,
            border: false,
            padding: draw::DEFAULT_PADDING,
            margin: 0,
        }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) -> &mut Self {
        self.widgets.push((widget, Cell::new(Rect::zero())));
        self
    }
    pub fn set_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }
    pub fn set_border(&mut self, border: bool) -> &mut Self {
        self.border = border;
        self
    }
    pub fn set_padding(&mut self, padding: u32) -> &mut Self {
        self.padding = padding;
        self
    }
    pub fn set_margin(&mut self, margin: u32) -> &mut Self {
        self.margin = margin;
        self
    }
}

impl Widget for WBox {
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_widgets(self.orientation, self.padding, Some(Point::new(self.margin, self.margin)), &self.widgets);
        if self.margin > 0 {
            let bounds = ctx.bounds();
            ctx.claim(Rect::new(0, 0, bounds.width + self.margin, bounds.height + self.margin));
        }
        if self.border {
            let bounds = ctx.bounds();
            ctx.draw_rect(Rect::new(0, 0, bounds.width, bounds.height));
        }
    }
    fn process_event(&mut self, e: &Event, bounds: Rect) -> bool {
        let mut draw = false;
        for (widget, rect) in &self.widgets {
            let child_bounds = bounds.point() + rect.get();
            draw |= widget.borrow_mut().process_event(e, child_bounds);
        }
        draw
    }
}
