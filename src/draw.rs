use crate::prelude::*;

/**
 * The DrawContext has the context necessary for a widget to draw itself.
 * The widget may be unaware of its placement. It can just use the zero point
 * to draw from the top-left corner. The offset is added before sending the commands to the
 * actual rendering backend.
*/
pub struct DrawContext<'a, B> where B: DrawBackend {
    backend: &'a mut B,
    bounds: Rect,
}


impl<'a, B> DrawContext<'a, B> where B: DrawBackend {
    pub fn draw_rect(&mut self, rect: Rect) {
        self.backend.draw_rect(rect);
    }
    /// Get the draw bounds of this context.
    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }
}
