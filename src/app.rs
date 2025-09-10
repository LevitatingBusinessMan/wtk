use std::cmp;

use crate::prelude::*;
use crate::{event::Event, widgets::SharedWidget};
use crate::backends::Backend;


pub struct App<B> where B: Backend {
    widgets: Vec<SharedWidget>,
    backend: B,
    quit: bool,
}

impl<B> App<B> where B: Backend {
    pub fn new(title: &'static str) -> Self {
        App { 
            widgets: vec![],
            backend: B::init(title),
            quit: false,
        }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) {
        self.widgets.push(widget);
    }
    pub fn process_event(&mut self, e: &Event) -> bool {
        let mut draw = false;
        match e {
            Event::Quit => {
                self.quit = true;
                return false;
            },
            Event::Resized(_) => { draw = true },
            _ => {},
        }
        for widget in &self.widgets {
            if widget.borrow_mut().process_event(e) {
                draw = true;
            }
        }
        draw
    }
    pub fn run(&mut self) {
        self.draw();
        while !self.quit {
            if let Some(e) = self.backend.poll_event() {
                let draw = self.process_event(&e);
                if draw { self.draw(); }
            }
        }
    }
    fn draw(&mut self) {
        let backend = self.backend.draw_backend();
        backend.clear();
        let padding = 5;
        let mut cursor = Point::new(padding, padding);
        let mut window_size = Size::new(0, 0);
        for widget in &self.widgets {
            let widget = widget.borrow_mut();
            let mut ctx = DrawContext::new(&*widget, cursor);
            widget.draw(&mut ctx);
            let bounds = ctx.bounds();
            cursor.y += bounds.height + padding; // move down
            window_size.height = cmp::max(window_size.height, bounds.total().height);
            window_size.width = cmp::max(window_size.width, bounds.total().width);
            ctx.run_backend(backend);
        }
        backend.present();
        self.backend.resize(window_size + padding);
    }
}

