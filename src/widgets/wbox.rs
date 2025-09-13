use crate::prelude::*;
use super::SharedWidget;
use crate::draw;

/// Widget Box for grouping widgets
pub struct WBox {
    widgets: Vec<SharedWidget>,
    orientation: Orientation,
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
            widgets,
            orientation,
            border: false,
            padding: draw::DEFAULT_PADDING,
            margin: 0,
        }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) -> &mut Self {
        self.widgets.push(widget);
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
        draw::draw_widgets(ctx, self.orientation, self.padding, &self.widgets, Some(Point::new(self.margin, self.margin)));
        if self.margin > 0 {
            let bounds = ctx.bounds();
            ctx.claim(Rect::new(0, 0, bounds.width + self.margin, bounds.height + self.margin));
        }
        if self.border {
            let bounds = ctx.bounds();
            ctx.draw_rect(Rect::new(0, 0, bounds.width, bounds.height));
        }
    }
    fn process_event(&mut self, event: &Event) -> bool { 
        let mut draw = false;
        for widget in &self.widgets {
            if widget.borrow_mut().process_event(event) {
                draw = true
            }
        }
        draw
    }  
}
