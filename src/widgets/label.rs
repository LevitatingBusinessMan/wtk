use crate::prelude::*;

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
        ctx.draw_text(&self.text, ctx.zero_point());
    }
}
