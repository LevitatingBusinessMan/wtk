//! Home of most drawing functions.
//! 
//! The drawing process is a bit convoluted.
//! The widgets require dyn compatibility so they can be grouped heterogenously.
//! This means that they cannot interact with generics through their [Widget] trait methods.
//! wtk requires generics to do any backend interactions.
//! 
//! So the process is this, the widget interacts with [DrawContext] to create a list of [DrawCommand]s.
//! These [DrawCommand]s are then converted by [DrawContext::run_backend] to [DrawBackend] methods.
//! 
//! Some Widgets need to be aware of their bounds (the area they occupy) so they can respond to the cursor.
//! This information is typically given to them during their draw time.
//! This means that currently, you cannot move a [DrawContext] draw location.
//! If you do want to do so, you must update the bounds information for each moved widget.
//! This is how the [Centered] widget works.

use std::{cmp, rc::Rc};

use crate::{fonts, prelude::*, rect::{Alignment, Orientation}, theme, widgets::ChildWidget};

pub const DEFAULT_SPACING: u32 = 5;

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
#[derive(Debug, Clone)]
pub struct DrawContext {
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

    pub fn new() -> Self {
        Self {
            commands: vec![
                DrawCommand::Color(theme::THEME.primary)
            ],
        }
    }

    /// The bounds of all the combined draw commands,
    /// used to generate the next position in the layout.
    pub fn size(&self) -> Size {
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
                    let rect = point.with_size(fonts::monogram::text_size(text));
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
        max
    }
    
    /// For drawing many widgets at ones. This may be used by container like widgets.
    /// Child widgets each get a DrawContext of which the commands are merged with the parents.
    /// Each widget has a corresponding Cell in which the relative bounds are stored.
    #[deprecated]
    pub fn draw_widgets(&mut self, orientation: Orientation, spacing: u32, at: Option<Point>, widgets: &[ChildWidget]) {
        let mut cursor = at.unwrap_or(Point::zero());
        for child in widgets {
            let mut child_ctx = DrawContext::new();
            child.widget.borrow().draw(&mut child_ctx);
            let size = child_ctx.size();
            child.bounds.set(cursor.with_size(size));
            self.merge_at(cursor, child_ctx);
            match orientation {
                Orientation::Horizontal => cursor.x += size.width + spacing,
                Orientation::Vertical => cursor.y += size.height + spacing,
            }
        }
    }

    /// Draw any numer of widgets in a specific orientation and alignment.
    /// This can be used for container widgets to draw children.
    /// Takes a slice of [ChildWidget]s, in which the widget bounds will be stored.
    pub fn draw_widgets_aligned(&mut self, orientation: Orientation, alignment: Alignment, spacing: u32, at: Option<Point>, widgets: &[ChildWidget]) {
        // some terminology from https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/CSS_layout/Flexbox#the_flex_model
        let mut cursor = at.unwrap_or(Point::zero());
        let children: Vec<(DrawContext, Size)> = widgets.iter().map(|child| {
            let mut ctx = DrawContext::new();
            child.widget.borrow().draw(&mut ctx);
            let size = ctx.size();
            (ctx, size)
        }).collect();
        let max_cross_size = children.iter().map(|(_, size)| match orientation {
            Orientation::Horizontal => size.height,
            Orientation::Vertical => size.width,
        }).max().unwrap_or(0);
        for ((child_ctx, size), child) in children.into_iter().zip(widgets.iter()) {
            let this_cross_size = match orientation {
                Orientation::Horizontal => size.height,
                Orientation::Vertical => size.width,
            };
            let cross_offset = match alignment {
                Alignment::Start => 0,
                Alignment::Center => (max_cross_size - this_cross_size) / 2,
                Alignment::End => max_cross_size - this_cross_size,
            };
            let pos = cursor + match orientation {
                Orientation::Horizontal => Point::new(0, cross_offset),
                Orientation::Vertical => Point::new(cross_offset, 0),
            };
            self.merge_at(pos, child_ctx);
            child.bounds.set(pos.with_size(size));
            match orientation {
                Orientation::Horizontal => cursor.x += size.width + spacing,
                Orientation::Vertical => cursor.y += size.height + spacing,
            }
        }
    }

    /// empty drawing operating for increasing the claimed bounds
    pub fn claim(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::Claim(rect));
    } 
}

/// Internal methods for a [DrawContext]. May be exposed manually via this trait.
pub trait DrawContextInternal {
    // /// Merge the commands of another [DrawContext] into this one.
    // fn merge(&mut self, ctx: DrawContext);
    /// Execute all draw commands using a [DrawBackend]. 
    fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend;
    // /// Update the zero point. This is may lead to unexpected bugs,
    // /// be careful not to mess up the bounds information for the widgets.
    // fn set_zero_point(&mut self, point: Point);
    /// Place the draw commands of another [DrawContext] into this one,
    /// mapping them to the location at `at`.
    fn merge_at(&mut self, at: Point, ctx: DrawContext);
    /// Add the coordinates of a point `at` to all draw commands,
    /// essentially shifting the drawing to point `at`.
    fn move_to(&mut self, at: Point);
}

impl DrawContextInternal for DrawContext {
    fn run_backend<B>(&self, backend: &mut B) where B: DrawBackend  {
        for command in &self.commands {
            match command {
                DrawCommand::Rect(rect) => backend.draw_rect(*rect),
                DrawCommand::Color(color) => backend.set_color(*color),
                DrawCommand::Text(text, point) => backend.draw_text(text, *point),
                DrawCommand::Claim(_) => {},
                DrawCommand::FillRect(rect) => backend.fill_rect(*rect),
            }
        }
    }
    fn merge_at(&mut self, at: Point, ctx: DrawContext) {
        let mut ctx = ctx.clone();
        ctx.move_to(at);
        self.commands.append(&mut ctx.commands);
    }
    fn move_to(&mut self, at: Point) {
        for command in &mut self.commands {
            *command = match command {
                DrawCommand::Rect(rect) => DrawCommand::Rect(at + *rect),
                DrawCommand::Text(str, point) => DrawCommand::Text(str.clone(), at + *point),
                DrawCommand::Claim(rect) => DrawCommand::Claim(at + *rect),
                DrawCommand::FillRect(rect) => DrawCommand::FillRect(at + *rect),
                _ => command.clone()
            };
        }
    }
    // fn merge(&mut self, ctx: DrawContext) {
    //     if ctx.zero_point() < self.zero_point() {
    //         panic!()
    //     }
    //     let diff = ctx.zero_point().abs_diff(self.zero_point);
    //     for command in ctx.commands {
    //         self.commands.push(match command {
    //             DrawCommand::Rect(rect) => DrawCommand::Rect(diff + rect),
    //             DrawCommand::Text(str, point) => DrawCommand::Text(str.clone(), diff + point),
    //             DrawCommand::Claim(rect) => DrawCommand::Claim(diff + rect),
    //             DrawCommand::FillRect(rect) => DrawCommand::FillRect(diff + rect),
    //             _ => command.clone()
    //         });
    //     }
    // }
    // fn set_zero_point(&mut self, point: Point) {
    //     self.zero_point = point;
    // }
}
