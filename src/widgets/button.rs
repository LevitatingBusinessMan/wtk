use std::rc::Rc;

use crate::{font, prelude::*};

pub struct Button {
    text: String,
    cb: Option<Rc<dyn Fn(&mut Button)>>,
    bounds: Rect,
}

impl Button {
    pub fn new<F>(text: impl Into<String>, cb: F) -> Button where F: Fn(&mut Button) + 'static{
        Button {
            text: text.into(),
            cb: Some(Rc::new(cb)),
            bounds: Rect::zero(),
        }
    }
    pub fn on_click<F>(&mut self, cb: F) -> &mut Self where F: Fn(&mut Button) + 'static {
        self.cb = Some(Rc::new(cb));
        self
    }
    pub fn set_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }
}

impl Widget for Button {

    fn process_event(&mut self, e: &Event) -> bool {
        match e {
            Event::MouseButtonDown { button: _, clicks: _, pos } => {
                if pos.is_in(self.bounds) {
                    if let Some(cb) = &self.cb {
                        let cb = cb.clone();
                        cb(self);
                        return true;
                    }
                }
            },
            _ => {}
        }
        return false;
    }
    
    fn draw(&self, ctx: &mut DrawContext) {
        let text_size = font::text_size(&self.text);
        let padding = 6;
        ctx.draw_text(&self.text, Point::new(padding, padding));
        ctx.draw_rect(Point::zero().with_size(text_size + padding * 2));
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    // fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    // fn as_any(&self) -> &dyn std::any::Any { self }
}
