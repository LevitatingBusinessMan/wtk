use crate::prelude::*;

#[derive(Debug)]
pub enum Event {
    Quit,
    Unsupported,
    MouseButtonDown{
        button: input::MouseButton,
        pos: Point,
    },
    MouseButtonUp{
        button: input::MouseButton,
        pos: Point,
    },
    Resized(Size),
    MouseMove(Point),
    TextInput(String),
}

pub mod input {
    #[derive(Debug)]
    pub enum MouseButton {
        Left,
        Right,
        Middle
    }
}
