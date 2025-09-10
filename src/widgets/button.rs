use crate::prelude::*;

#[derive(Debug)]
pub struct Button {
    text: String,
    cb: Option<()>,
}

impl Button {
    pub fn new<T: Fn() -> ()>(text: impl Into<String>, cb: T) -> Button {
        Button { text: text.into(), cb: None }
    }
    pub fn on_click(&mut self, cb: ()) -> &mut Self {
        self.cb = Some(());
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
    
    fn draw<B>(&self, ctx: &mut DrawContext<B>) where B: DrawBackend {
        ctx.draw_rect(Rect::new(0, 0, 30, 10));
    }
    
    fn size(&self) -> (u32,u32) {
        return (30, 10);
    }
}
