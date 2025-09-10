use crate::prelude::*;
use crate::{event::Event, widgets::SharedWidget};
use crate::Backend;


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
    pub fn process_event(&mut self, e: &Event) {
        match e {
            Event::Quit => {
                self.quit = true;
                return;
            }
            _ => {},
        }
        for widget in &self.widgets {
            widget.borrow_mut().process_event(e);
        }
    }
    pub fn run(&mut self) {
        while !self.quit {
            if let Some(e) = self.backend.poll_event() {
                self.process_event(&e);
                self.draw();
            }
        }
    }
    fn draw(&mut self) {
        let backend = self.backend.draw_backend();
        backend.clear();
        for widget in &self.widgets {
            let widget = widget.borrow_mut();
            let mut ctx = DrawContext::new(&*widget, Position::zero());
            widget.draw(&mut ctx);
            ctx.run_backend(backend);
        }
        backend.present();
    }
}

