use crate::prelude::*;

#[derive(Debug)]
pub enum Event {
    Quit,
    Unsupported,
    MouseButtonDown{
        button: input::MouseButton,
        clicks: u8,
        pos: Point,
    },
    Resized(Size),
}

pub mod input {
    #[derive(Debug)]
    pub enum MouseButton {
        Left,
        Right,
        Middle
    }
}
