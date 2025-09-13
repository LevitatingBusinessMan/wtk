use std::ops;
use std::cmp;
use std::ops::AddAssign;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    /// width + x, height + x
    pub fn total(self) -> Size {
        Size {
            width: self.width + self.x,
            height: self.height + self.y,
        }
    }
    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }
    pub fn size(self) -> Size {
        Size::new(self.width, self.height)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn with_size(self, size: Size) -> Rect {
        Rect::new(self.x, self.y, size.width, size.height)
    }
    pub fn zero() -> Self {
        Point::new(0,0)
    }
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
    pub fn is_in(self, rect: Rect) -> bool {
        return self.x >= rect.x && self.x < rect.total().width && self.y >= rect.y && self.y < rect.total().height
    }
}

impl ops::Add<Rect> for Point {
    type Output = Rect;

    fn add(self, rhs: Rect) -> Self::Output {
        Rect {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            width: rhs.width,
            height: rhs.height
        }
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point> for Point {
    type Output = Point;
    
    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl cmp::PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let xord = self.x.cmp(&other.x);
        let yord = self.y.cmp(&other.y);
        if xord == yord {
            return Some(xord)
        } else {
            return None
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

/// add to width and height
impl std::ops::Sub<u32> for Size {    
    type Output = Size;
    
    fn sub(self, rhs: u32) -> Self::Output {
        Self {
            width: self.width - rhs,
            height: self.height - rhs,
        }
    }
}

impl AddAssign<u32> for Size {
    fn add_assign(&mut self, rhs: u32) {
        self.width += rhs;
        self.height += rhs;
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

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal, Vertical
}
