use std::rc::Rc;

use crate::{font, prelude::*};

pub struct Label {
    text: String,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
    pub fn set_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }
}

impl Widget for Label {
    fn draw(&self, ctx: &mut DrawContext) {
        let padding = 6;
        ctx.draw_text(&self.text, Point::new(padding, padding));
    }
}
