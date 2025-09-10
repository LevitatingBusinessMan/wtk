use crate::prelude::*;

pub struct Button {
    text: String,
    cb: Option<Box<dyn Fn(&mut Button)>>,
}

impl Button {
    pub fn new<T: Fn(&mut Button) -> ()>(text: impl Into<String>, cb: T) -> Button {
        Button { text: text.into(), cb: None }
    }
    pub fn on_click(&mut self, cb: &dyn Fn(Button)) -> &mut Self {
        self.cb = None;
        self
    }
    pub fn set_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.text = text.into();
        self
    }
}

impl Widget for Button {

    fn process_event(&mut self, e: &Event) {
    }
    
    fn draw(&self, ctx: &mut DrawContext) {
        ctx.draw_rect(Position::zero().with_size(&self.size()));
    }
    
    fn size(&self) -> Size {
        return Size::new(50, 30)
    }
}
