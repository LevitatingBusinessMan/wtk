#[derive(Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn with_size(&self, size: Size) -> Rect {
        Rect::new(self.x, self.y, size.width, size.height)
    }
    pub fn zero() -> Self {
        Point::new(0,0)
    }
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl std::ops::Mul<Size> for f64 {
    type Output = Size;

    fn mul(self, rhs: Size) -> Self::Output {
        Size {
            width: ((rhs.width as f64) * self).round() as u32,
            height: ((rhs.height as f64) * self).round() as u32
        }
    }
}

impl Size {
    pub const fn new(width: u32, height: u32) -> Self {
        Size {
            width,
            height
        }
    }
}

