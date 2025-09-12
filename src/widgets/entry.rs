use crate::prelude::*;

pub struct Entry {
    text: String,
    focus: bool,
}

impl Entry {
    pub fn new() -> Self {
        Self { text: String::new(), focus: true }
    }
    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
}

impl Widget for Entry {
    fn draw(&self, ctx: &mut DrawContext) {
        let padding = 6;
        ctx.draw_text(&self.text, Point::new(padding, padding));
    }
}
