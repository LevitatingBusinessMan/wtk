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
    MouseButtonUp{
        button: input::MouseButton,
        clicks: u8,
        pos: Point,
    },
    Resized(Size),
    MouseMove(Point),
}

pub mod input {
    #[derive(Debug)]
    pub enum MouseButton {
        Left,
        Right,
        Middle
    }
}
