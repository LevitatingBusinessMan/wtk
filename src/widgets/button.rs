use std::rc::Rc;

use crate::prelude::*;
use crate::theme;

pub struct Button {
    text: String,
    cb: Option<Rc<dyn Fn(&mut Button)>>,
    bounds: Rect,
    pressed: bool,
    padding: u32,
}

impl Button {
    pub fn new<F>(text: impl Into<String>, cb: F) -> Button where F: Fn(&mut Button) + 'static{
        Button {
            text: text.into(),
            cb: Some(Rc::new(cb)),
            bounds: Rect::zero(),
            pressed: false,
            padding: 6,
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
    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }
}

impl Widget for Button {

    fn process_event(&mut self, e: &Event) -> bool {
        match e {
            Event::MouseButtonDown { button: _b, pos } => {
                if pos.is_in(self.bounds) && matches!(_b, MouseButton::Left) {
                    println!("{:?} clicked", self.bounds);
                    self.pressed = true;
                    if let Some(cb) = &self.cb {
                        let cb = cb.clone();
                        cb(self);
                        return true;
                    }
                }
            },
            Event::MouseButtonUp { button: _b, pos } => {
                if matches!(_b, MouseButton::Left) {
                    self.pressed = false;
                    return true;
                }
            },
            _ => {}
        }
        return false;
    }
    
    fn draw(&self, ctx: &mut DrawContext) {
        let text_size = font::text_size(&self.text);
        let mut button_size = text_size + self.padding * 2;
        if self.pressed {
            button_size += 2;
        }

        ctx.set_color(theme::THEME.interactive);
        ctx.fill_rect(Point::zero().with_size(button_size));
        ctx.draw_text(&self.text, Point::new(self.padding, self.padding));
        ctx.set_color(theme::THEME.primary);
        ctx.draw_rect(Point::zero().with_size(button_size));

        if !self.pressed {
            //shadow
            ctx.draw_rect(Point::zero().with_size(button_size + 1));
            ctx.draw_rect(Point::zero().with_size(button_size + 2));
        }
    }
    
    fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    // fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
    // fn as_any(&self) -> &dyn std::any::Any { self }
}
