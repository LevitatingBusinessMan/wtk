use std::rc::Rc;

use crate::{draw, font, prelude::*};

use super::SharedWidget;

pub struct HBox {
    widgets: Vec<SharedWidget>,
}

impl HBox {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) -> &mut Self {
        self.widgets.push(widget);
        self
    }
}

impl Widget for HBox {
    fn draw(&self, ctx: &mut DrawContext) {
        draw::draw_widgets(ctx, Orientation::Horizontal, 5, &self.widgets);
    }
}
