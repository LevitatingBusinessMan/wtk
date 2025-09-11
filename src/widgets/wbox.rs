use crate::prelude::*;
use super::SharedWidget;
use crate::draw;

pub struct WBox {
    widgets: Vec<SharedWidget>,
    orientation: Orientation,
    padding: u32,
    border: bool,
}

impl WBox {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            widgets: vec![],
            orientation: orientation,
            border: false,
            padding: 5,
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
}

impl Widget for WBox {
    fn draw(&self, ctx: &mut DrawContext) {
        draw::draw_widgets(ctx, Orientation::Horizontal, self.padding, &self.widgets);
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
