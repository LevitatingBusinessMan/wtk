//! The drawing process is a bit convoluted.
//! The widgets require dyn compatibility so they can be grouped heterogenously.
//! This means that they cannot interact with generics through their [Widget] trait methods.
//! wtk requires generics to do any backend interactions.
//! 
//! So the process is this, the widget interacts with [DrawContext] to create a list of [DrawCommand]s.
//! These [DrawCommand] are then converted by [DrawContext::run_backend] to [DrawBackend] methods.

use crate::prelude::*;

#[derive(Debug)]
enum DrawCommand {
    Rect(Rect),
    Color(Color),
    Text(String, Point)
}

/**
 * The DrawContext has the context necessary for a widget to draw itself.
 * The widget may be unaware of its placement. It can just use the zero point
 * to draw from the top-left corner. The offset is added before sending the commands to the
 * actual rendering backend.
*/
pub struct DrawContext {
    bounds: Rect,
    commands: Vec<DrawCommand>,
}


impl DrawContext {
    pub fn draw_rect(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::Rect(rect));
    }
    pub fn draw_text<T: Into<String>>(&mut self, text: T, pos: Point) {
        self.commands.push(DrawCommand::Text(text.into(), pos));
    }
    /// Get the draw bounds of this context.
    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }
    pub(crate) fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend  {
        eprintln!("{:?}", self.commands);
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => backend.draw_rect(rect),
                DrawCommand::Color(color) => backend.set_color(*color),
                DrawCommand::Text(text, point) => backend.draw_text(text, *point)
            }
        }
    }
    pub(crate) fn new(widget: &dyn Widget, pos: Point) -> Self {
        Self {
            bounds: pos.with_size(widget.size()),
            commands: vec![
                DrawCommand::Color(Color::RGB(0, 0, 0))
            ],
        }
    }
}
