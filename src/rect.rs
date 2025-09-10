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

impl std::ops::Mul<f64> for Size {
    type Output = Size;

    fn mul(self, rhs: f64) -> Self::Output {
        Size {
            width: ((self.width as f64) * rhs).round() as u32,
            height: ((self.height as f64) * rhs).round() as u32
        }
    }
}

/// add to width and height
impl std::ops::Add<u32> for Size {    
    type Output = Size;
    
    fn add(self, rhs: u32) -> Self::Output {
        Self {
            width: self.width + rhs,
            height: self.height + rhs,
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

