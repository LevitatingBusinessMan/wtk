//! Home of most drawing functions.
//! 
//! The drawing process is a bit convoluted.
//! The widgets require dyn compatibility so they can be grouped heterogenously.
//! This means that they cannot interact with generics through their [Widget] trait methods.
//! wtk requires generics to do any backend interactions.
//! 
//! So the process is this, the widget interacts with [DrawContext] to create a list of [DrawCommand]s.
//! These [DrawCommand]s are then converted by [DrawContext::run_backend] to [DrawBackend] methods.

use std::{cmp, rc::Rc};

use crate::{font, prelude::*, rect::Orientation, theme, widgets::SharedWidget};

pub const DEFAULT_PADDING: u32 = 5;

#[derive(Debug, Clone)]
enum DrawCommand {
    Rect(Rect),
    Color(Color),
    Text(Rc<String>, Point),
    Claim(Rect),
    FillRect(Rect),
}

/**
 * The DrawContext has the context necessary for a widget to draw itself.
 * The widget may be unaware of its placement. It can just use the zero point
 * to draw from the top-left corner. The offset is added before sending the commands to the
 * actual rendering backend.
 * 
 * DrawContext always sets a default color.
*/
#[derive(Debug)]
pub struct DrawContext {
    zero_point: Point,
    commands: Vec<DrawCommand>,
    //hover: bool,
}

impl DrawContext {
    pub fn draw_rect(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::Rect(rect));
    }
    pub fn draw_text<T: Into<String>>(&mut self, text: T, pos: Point) {
        self.commands.push(DrawCommand::Text(Rc::new(text.into()), pos));
    }
    pub fn set_color(&mut self, color: Color) {
        self.commands.push(DrawCommand::Color(color));
    }
    pub fn fill_rect(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::FillRect(rect));
    }
    /// Get the draw origin
    pub fn zero_point(&self) -> Point {
        self.zero_point
    }

    pub fn new(pos: Point) -> Self {
        Self {
            zero_point: pos,
            commands: vec![
                DrawCommand::Color(theme::THEME.primary)
            ],
        }
    }

    /// The bounds of all the combined draw commands,
    /// used to generate the next position in the layout.
    pub fn bounds(&self) -> Rect {
        // the maximum drawpoint reached from the zero point
        let mut max = Size::new(0, 0);
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => {
                    max.width = cmp::max(max.width, rect.total().width);
                    max.height = cmp::max(max.height, rect.total().height);
                },
                DrawCommand::Color(_color) => {},
                DrawCommand::Text(text, point) => {
                    let rect = point.with_size(font::text_size(text));
                    max.width = cmp::max(max.width, rect.total().width);
                    max.height = cmp::max(max.height, rect.total().height);
                },
                DrawCommand::Claim(rect) => {
                    max.width = cmp::max(max.width, rect.total().width);
                    max.height = cmp::max(max.height, rect.total().height);
                },
                DrawCommand::FillRect(rect) => {
                    max.width = cmp::max(max.width, rect.total().width);
                    max.height = cmp::max(max.height, rect.total().height);
                }
            }
        }
        self.zero_point.with_size(max)
    }
    
    /// For drawing many widgets at ones. This may be used by container like widgets.
    /// Child widgets each get a DrawContext of which the commands are merged with the parents.
    pub fn draw_widgets(&mut self, orientation: Orientation, padding: u32,  at: Option<Point>, widgets: &[SharedWidget]) {
        // the cursor is where we start drawing the next widget
        let mut cursor = at.map_or(self.zero_point(), |at| at + self.zero_point());
        for widget in widgets {
            let mut child_ctx = DrawContext::new(cursor);
            widget.borrow().draw(&mut child_ctx);
            let bounds = child_ctx.bounds();
            widget.borrow_mut().set_bounds(bounds);
            match orientation {
                Orientation::Horizontal => cursor.x += bounds.width + padding,
                Orientation::Vertical => cursor.y += bounds.height + padding,
            }
            self.merge(child_ctx);
        }
    }
    
    /// empty drawing operating for increasing the claimed bounds
    pub fn claim(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::Claim(rect));
    } 
}

/// Internal methods for a [DrawContext]. May be exposed manually via this trait.
pub trait DrawContextInternal {
    /// Merge the commands of another [DrawContext] into this one.
    fn merge(&mut self, ctx: DrawContext);
    /// Execute all draw commands using a [DrawBackend]. 
    fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend;
    /// Update the zero point. This is may lead to unexpected bugs,
    /// be careful not to mess up the bounds information for the widgets.
    fn set_zero_point(&mut self, point: Point);
}

impl DrawContextInternal for DrawContext {
    fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend  {
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => backend.draw_rect(self.zero_point() + *rect),
                DrawCommand::Color(color) => backend.set_color(*color),
                DrawCommand::Text(text, point) => backend.draw_text(text, self.zero_point() + *point),
                DrawCommand::Claim(_) => {},
                DrawCommand::FillRect(rect) => backend.fill_rect(self.zero_point() + *rect),
            }
        }
    }
    /// Merge the commands of another [DrawContext] into this one.
    fn merge(&mut self, ctx: DrawContext) {
        if ctx.zero_point() < self.zero_point() {
            panic!()
        }
        let diff = ctx.zero_point() - self.zero_point();
        for command in ctx.commands {
            self.commands.push(match command {
                DrawCommand::Rect(rect) => DrawCommand::Rect(diff + rect),
                DrawCommand::Text(str, point) => DrawCommand::Text(str.clone(), diff + point),
                DrawCommand::Claim(rect) => DrawCommand::Claim(diff + rect),
                DrawCommand::FillRect(rect) => DrawCommand::FillRect(diff + rect),
                _ => command.clone()
            });
        }
    }
    fn set_zero_point(&mut self, point: Point) {
        self.zero_point = point;
    }
}
