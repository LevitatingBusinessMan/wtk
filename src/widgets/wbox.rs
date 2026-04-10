use crate::{prelude::*, widgets::ChildWidget};
use super::SharedWidget;
use crate::draw;

/// Widget Box for grouping widgets
pub struct WBox {
    widgets: Vec<ChildWidget>,
    orientation: Orientation,
    alignment: Alignment,
    spacing: u32,
    padding: u32,
    border: bool,
}

impl WBox {
    // pub fn new(orientation: Orientation) -> Self {
    //     Self {
    //         widgets: vec![],
    //         orientation,
    //         border: false,
    //         spacing: draw::DEFAULT_SPACING,
    //         padding: 0,
    //         alignment: Alignment::Start,
    //     }
    // }
    pub fn horizontal() -> Self {
        Self {
            widgets: vec![],
            orientation: Orientation::Horizontal,
            border: false,
            spacing: draw::DEFAULT_SPACING,
            padding: 0,
            alignment: Alignment::Start,
        }
    }
    pub fn vertical() -> Self {
        Self {
            widgets: vec![],
            orientation: Orientation::Vertical,
            border: false,
            spacing: draw::DEFAULT_SPACING,
            padding: 0,
            alignment: Alignment::Start,
        }
    }
    pub fn with(mut self, widgets: Vec<SharedWidget>) -> Self {
        self.widgets = widgets.into_iter().map(ChildWidget::new).collect();
        self
    }
    pub fn align(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }
    pub fn add_widget(&mut self, widget: SharedWidget) -> &mut Self {
        self.widgets.push(ChildWidget::new(widget));
        self
    }
    #[deprecated]
    pub fn set_orientation(&mut self, orientation: Orientation) -> &mut Self {
        self.orientation = orientation;
        self
    }
    #[deprecated]
    pub fn set_border(&mut self, border: bool) -> &mut Self {
        self.border = border;
        self
    }
    #[deprecated]
    pub fn set_spacing(&mut self, spacing: u32) -> &mut Self {
        self.spacing = spacing;
        self
    }
    #[deprecated]
    pub fn set_padding(&mut self, padding: u32) -> &mut Self {
        self.padding = padding;
        self
    }
}

impl Widget for WBox {
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_widgets_aligned(self.orientation, self.alignment, self.spacing, Some(Point::new(self.padding, self.padding)), &self.widgets);
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
