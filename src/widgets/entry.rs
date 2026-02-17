use std::cmp;

use crate::prelude::*;

pub struct Entry {
    text: String,
    focus: bool,
    min_width: u32,
    bounds: Rect,
}

impl Entry {
    pub fn new() -> Self {
        Self { text: String::new(), focus: false, min_width: 80, bounds: Rect::zero() }
    }
    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
    pub fn get_text(&self) -> &str {
        &self.text
    }
    pub fn set_min_width(&mut self, width: u32) -> &mut Self {
        self.min_width = width;
        self
    }
}

impl Widget for Entry {
    fn draw(&self, ctx: &mut DrawContext) {
        let padding = (2.0 * font::scale()) as u32;
        ctx.draw_text(&self.text, Point::new(padding, padding));
        let text_size = font::text_size(&self.text);
        let width = cmp::max(text_size.width, self.min_width) + padding;
        ctx.draw_rect(Point::zero().with_size(Size::new(width, text_size.height + padding)));
        if self.focus {
            ctx.draw_rect(Point::new(1,1).with_size(Size::new(width, text_size.height + padding) - 2));
        }
    }
    
    fn process_event(&mut self, event: &Event, bounds: Rect) -> bool {
        match event {
            Event::MouseButtonDown { button: _b, pos } => {
                let clicked = pos.is_in(bounds);
                let focus_prev = self.focus;
                self.set_focus(clicked);
                clicked || focus_prev != self.focus
            },
            Event::MouseButtonUp { button: _b, pos } => {
                let clicked = pos.is_in(bounds);
                let focus_prev = self.focus;
                self.set_focus(clicked);
                clicked || focus_prev != self.focus
            },
            Event::TextInput(text) => {
                if self.focus {
                    self.text += text;
                    true
                } else {
                    false
                }
            },
            _ => false,
        }
    }
}
