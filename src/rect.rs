#[derive(Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            w: width,
            h: height,
        }
    }
}


pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn with_size(&self, size: &Size) -> Rect {
        Rect::new(self.x, self.y, size.width, size.height)
    }
    pub fn zero() -> Self {
        Position::new(0,0)
    }
    pub fn new(x: u32, y: u32) -> Self {
        Position { x, y }
    }
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Size {
            width,
            height
        }
    }
}

