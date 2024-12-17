use std::ops::{Mul, Sub, Add};

use super::Point;

#[derive(Default,Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct IPoint {
	pub x : i32,
	pub y : i32,
}

impl IPoint {
	pub fn new( x : i32, y : i32 ) -> Self {
		Self {
			x,
			y,
		}
	}

    pub fn from_point(point: Point) -> Self {
        Self {
            x: point.x as i32,
            y: point.y as i32,
        }
    }

	pub fn manhatten_dist( &self, to : IPoint ) -> i32 {
		(to.x - self.x).abs() + (to.y - self.y).abs()
	}

	pub fn dir_to( &self, to : IPoint ) -> IPoint {
		let off = to - *self;
		IPoint {
			x: off.x.signum(),
			y: off.y.signum(),
		}
	}

    pub fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < width as i32 && self.y >= 0 && self.y < height as i32
    }
}

impl Add<IPoint> for IPoint {
	type Output = IPoint;
	fn add(self, o : IPoint) -> Self {
		IPoint {
			x: self.x + o.x,
			y: self.y + o.y,
		}
	}
}

impl Sub<IPoint> for IPoint {
	type Output = IPoint;
	fn sub(self, o : IPoint) -> Self {
		IPoint {
			x: self.x - o.x,
			y: self.y - o.y,
		}
	}
}

impl Mul<i32> for IPoint {
	type Output = IPoint;
	fn mul(self, s : i32) -> Self {
		IPoint {
			x: self.x * s,
			y: self.y * s,
		}
	}
}

impl Mul<IPoint> for i32 {
	type Output = IPoint;
	fn mul(self, pt : IPoint) -> IPoint {
		IPoint {
			x: self * pt.x,
			y: self * pt.y,
		}
	}
}
