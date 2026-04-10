use crate::{prelude::*, widgets::ChildWidget};
use super::SharedWidget;
use crate::draw;

/// Widget Box for grouping widgets
pub struct WBox {
    widgets: Vec<ChildWidget>,
    orientation: Orientation,
    spacing: u32,
    padding: u32,
    border: bool,
}

impl WBox {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            widgets: vec![],
            orientation,
            border: false,
            spacing: draw::DEFAULT_SPACING,
            padding: 0,
        }
    }
    pub fn with(orientation: Orientation, widgets: Vec<SharedWidget>) -> Self {
        Self {
            widgets: widgets.into_iter().map(ChildWidget::new).collect(),
            orientation,
            border: false,
            spacing: draw::DEFAULT_SPACING,
            padding: 0,
        }
    }
    pub fn add_widget(&mut self, widget: SharedWidget) -> &mut Self {
        self.widgets.push(ChildWidget::new(widget));
        self
    }
    pub fn set_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }
    pub fn set_border(&mut self, border: bool) -> &mut Self {
        self.border = border;
        self
    }
    pub fn set_spacing(&mut self, spacing: u32) -> &mut Self {
        self.spacing = spacing;
        self
    }
    pub fn set_padding(&mut self, padding: u32) -> &mut Self {
        self.padding = padding;
        self
    }
}

impl Widget for WBox {
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_widgets(self.orientation, self.spacing, Some(Point::new(self.padding, self.padding)), &self.widgets);
        if self.border {
            let size = ctx.size();
            ctx.draw_rect(Point::zero().with_size(size + self.padding));
        }
    }
    fn process_event(&mut self, e: &Event, bounds: Rect) -> bool {
        let mut draw = false;
        for child in &self.widgets {
            let child_bounds = bounds.point() + child.bounds.get();
            draw |= child.widget.borrow_mut().process_event(e, child_bounds);
        }
        draw
    }
}
