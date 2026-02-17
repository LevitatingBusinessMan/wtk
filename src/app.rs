use std::cell::Cell;

use crate::{draw, prelude::*};
use crate::{event::Event, widgets::SharedWidget};
use crate::backends::Backend;
use crate::draw::DrawContextInternal;

pub struct App<B> where B: Backend {
    widgets: Vec<(SharedWidget, Cell<Rect>)>,
    backend: B,
    size: Size,
    pub quit: bool,
}

impl<B> App<B> where B: Backend {
    pub fn new(title: &str) -> Self {
        App { 
            widgets: vec![],
            backend: B::init(title),
            size: Size::zero(),
            quit: false,
        }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) {
        self.widgets.push((widget, Cell::new(Rect::zero())));
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
        for (widget, bounds) in &self.widgets {
            draw |= widget.borrow_mut().process_event(e, bounds.get());
        }
        draw
    }

    /// Continually process events, redrawing the UI if needed. Runs until a [Event::Quit] event is processed.
    /// If you need finer control over the run loop, use [App::poll_and_process_event] and [App::draw] directly.
    pub fn run(&mut self) {
        self.draw();
        while !self.quit {
            if let Some(e) = self.backend.poll_event() {
                let draw = self.process_event(&e);
                if draw { self.draw(); }
            }
        }
    }

    /// Executes [Backend::poll_event] on the backend. Then optionally process an event.
    /// If a widget requests a draw true is returned.
    pub fn poll_and_process_event(&mut self) -> bool {
        if let Some(e) = self.backend.poll_event() {
            self.process_event(&e)
        } else {
            false
        }
    }

    // fn draw(&mut self) {
    //     let backend = self.backend.draw_backend();
    //     backend.clear();
    //     let padding = 5;
    //     let mut cursor = Point::new(padding, padding);
    //     let mut window_size = Size::new(0, 0);
    //     for widget in &self.widgets {
    //         let mut widget = widget.borrow_mut();
    //         let mut ctx = DrawContext::new(cursor);
    //         widget.draw(&mut ctx);
    //         let bounds = ctx.bounds();
    //         widget.set_bounds(bounds);
    //         cursor.y += bounds.height + padding; // move down
    //         window_size.height = cmp::max(window_size.height, bounds.total().height);
    //         window_size.width = cmp::max(window_size.width, bounds.total().width);
    //         ctx.run_backend(backend);
    //     }
    //     backend.present();
    //     self.backend.resize(window_size + padding);
    // }

    /// Manually tell the backend to draw all widges. Useful for use in custom update loops.
    pub fn draw(&mut self) {
        let backend = self.backend.draw_backend();
        backend.clear();
        let padding = draw::DEFAULT_PADDING;
        let mut ctx = DrawContext::new(Point::new(padding, padding));
        ctx.draw_widgets(Orientation::Vertical, draw::DEFAULT_PADDING, None, &self.widgets);
        ctx.run_backend(backend);
        backend.present();
        let size = ctx.bounds().size();
        if self.size != size {
            self.size = size;
            self.backend.resize(size + padding * 2);
        }
    }
}
