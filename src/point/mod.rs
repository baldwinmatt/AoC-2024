use core::fmt;
use std::ops::{Add, Sub};

use crate::parse::Parseable;

pub mod ipoint;
pub mod traits;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn from_ipoint(ipoint: ipoint::IPoint) -> Self {
        Point {
            x: ipoint.x as usize,
            y: ipoint.y as usize,
        }
    }

    pub fn checked_sub(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_sub(rhs.x) {
            if let Some(y) = self.y.checked_sub(rhs.y) {
                return Some(Point::new(x, y));
            }
        }

        None
    }

    pub fn checked_add(&self, rhs: Self) -> Option<Self> {
        if let Some(x) = self.x.checked_add(rhs.x) {
            if let Some(y) = self.y.checked_add(rhs.y) {
                return Some(Point::new(x, y));
            }
        }

        None
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Iterator<Item = u8>> Parseable<Point> for T {
    fn next_number(&mut self) -> Option<Point> {
        if let Some((x, y)) = self.next_number().zip(self.next_number()) {
            return Some(Point::new(x, y));
        }
        None
    }
}
