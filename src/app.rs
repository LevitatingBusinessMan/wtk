use std::time::Duration;

use crate::widgets::ChildWidget;
use crate::{draw, prelude::*};
use crate::{event::Event, widgets::SharedWidget};
use crate::backends::Backend;
use crate::draw::DrawContextInternal;

pub const WTK_TARGET_FPS: f64 = 30.0;

pub struct App<B> where B: Backend {
    widgets: Vec<ChildWidget>,
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
        self.widgets.push(ChildWidget::new(widget));
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
        for child in &self.widgets {
            draw |= child.widget.borrow_mut().process_event(e, child.bounds.get());
        }
        draw
    }

    /// Continually process events, redrawing the UI if needed. Runs until a [Event::Quit] event is processed.
    /// If you need finer control over the run loop, use [App::poll_and_process_event] and [App::draw] directly.
    pub fn run(&mut self) {
        self.draw();
        while !self.quit {
            let mut draw = false;
            while let Some(e) = self.poll_event() {
                draw = self.process_event(&e) || draw;
            }
            if draw { self.draw(); }
            std::thread::sleep(Duration::from_secs_f64(1.0 / WTK_TARGET_FPS));
        }
    }

    pub(crate) fn poll_event(&mut self) -> Option<Event> {
        self.backend.poll_event()
    }

    // /// Executes [Backend::poll_event] on the backend. Then optionally process an event.
    // /// Returns None if no event was processed.
    // /// Otherwise returns if a draw is requested.
    // pub fn poll_and_process_event(&mut self) -> Option<bool> {
    //     if let Some(e) = self.backend.poll_event() {
    //         Some(self.process_event(&e))
    //     } else {
    //         None
    //     }
    // }

    /// Manually tell the backend to draw all widges. Useful for use in custom update loops.
    pub fn draw(&mut self) {
        self.backend.clear();
        let padding = draw::DEFAULT_SPACING;
        let mut ctx = DrawContext::new();
        ctx.draw_widgets(Orientation::Vertical, draw::DEFAULT_SPACING, Some(Point::new(padding, padding)), &self.widgets);
        ctx.run_backend(&mut self.backend);
        self.backend.present();
        let size = ctx.size();
        if self.size != size {
            self.size = size;
            self.backend.resize(size + padding);
        }
    }
}
