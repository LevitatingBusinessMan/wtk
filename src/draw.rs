//! The drawing process is a bit convoluted.
//! The widgets require dyn compatibility so they can be grouped heterogenously.
//! This means that they cannot interact with generics through their [Widget] trait methods.
//! wtk requires generics to do any backend interactions.
//! 
//! So the process is this, the widget interacts with [DrawContext] to create a list of [DrawCommand]s.
//! These [DrawCommand] are then converted by [DrawContext::run_backend] to [DrawBackend] methods.

use std::cmp;

use crate::{font, prelude::*};

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
    offset: Point,
    commands: Vec<DrawCommand>,
}


impl DrawContext {
    pub fn draw_rect(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::Rect(rect));
    }
    pub fn draw_text<T: Into<String>>(&mut self, text: T, pos: Point) {
        self.commands.push(DrawCommand::Text(text.into(), pos));
    }
    /// Get the draw offset
    pub fn offset(&self) -> Point {
        self.offset
    }
    pub(crate) fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend  {
        eprintln!("DrawCommands: {:?}", self.commands);
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => backend.draw_rect(self.offset + *rect),
                DrawCommand::Color(color) => backend.set_color(*color),
                DrawCommand::Text(text, point) => backend.draw_text(text, self.offset + *point)
            }
        }
    }
    pub(crate) fn new(widget: &dyn Widget, pos: Point) -> Self {
        Self {
            offset: pos,
            commands: vec![
                DrawCommand::Color(Color::RGB(0, 0, 0))
            ],
        }
    }
    /// The bounds of all the combined draw commands,
    /// used to generate the next position in the layout.
    pub(crate) fn bounds(&self) -> Rect {
        let mut max = Size::new(0, 0);
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => {
                    max.width = cmp::max(max.width, rect.width + rect.x);
                    max.height = cmp::max(max.height, rect.height + rect.y);
                },
                DrawCommand::Color(color) => {},
                DrawCommand::Text(text, point) => {
                    let rect = point.with_size(font::text_size(text));
                    max.width = cmp::max(max.width, rect.width + rect.x);
                    max.height = cmp::max(max.height, rect.height + rect.y);
                },
            }
        }
        self.offset.with_size(max)
    }
}
