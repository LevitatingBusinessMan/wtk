pub enum Event {
    Quit,
    Unsupported,
    MouseButtonDown{
        button: input::MouseButton,
        clicks: u8,
        x: u32,
        y: u32
    }
}

pub mod input {
    pub enum MouseButton {
        Left,
        Right,
        Middle
    }
}
