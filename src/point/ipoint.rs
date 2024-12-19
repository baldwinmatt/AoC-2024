use std::{ops::{Mul, Sub, Add, DivAssign, Div, MulAssign, SubAssign, AddAssign}, iter::Sum};
use forward_ref::{forward_ref_binop, forward_ref_op_assign};

use crate::parse::Parseable;

use super::{Point, traits::{ModuloPositiveAssign, ModuloPositive, ModuloAssign, Modulo, Absolute}};

#[derive(Default,Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct IPoint {
	pub x : isize,
	pub y : isize,
}

impl IPoint {
	pub fn new( x : isize, y : isize ) -> Self {
		Self {
			x,
			y,
		}
	}

    pub fn from_point(point: Point) -> Self {
        Self {
            x: point.x as isize,
            y: point.y as isize,
        }
    }

	pub fn manhatten_dist( &self, to : IPoint ) -> isize {
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
        self.x >= 0 && self.x < width as isize && self.y >= 0 && self.y < height as isize
    }
}

impl<T: Iterator<Item = u8>> Parseable<IPoint> for T {
    fn next_number(&mut self) -> Option<IPoint> {
        if let Some((x, y)) = self.next_number().zip(self.next_number()) {
            return Some(IPoint::new(x, y));
        }
        None
    }
}
impl Add for IPoint {
    type Output = IPoint;

    fn add(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<IPoint> for IPoint {
    fn add_assign(&mut self, rhs: IPoint) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
forward_ref_binop!(impl Add, add for IPoint, IPoint);
forward_ref_op_assign!(impl AddAssign, add_assign for IPoint, IPoint);

impl Sub for IPoint {
    type Output = IPoint;

    fn sub(self, rhs: Self) -> Self::Output {
        IPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<IPoint> for IPoint {
    fn sub_assign(&mut self, rhs: IPoint) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

forward_ref_binop!(impl Sub, sub for IPoint, IPoint);
forward_ref_op_assign!(impl SubAssign, sub_assign for IPoint, IPoint);

impl Mul<isize> for IPoint {
    type Output = IPoint;

    fn mul(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<isize> for IPoint {
    fn mul_assign(&mut self, rhs: isize) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

forward_ref_binop!(impl Mul, mul for IPoint, isize);
forward_ref_op_assign!(impl MulAssign, mul_assign for IPoint, isize);

impl Div<isize> for IPoint {
    type Output = IPoint;

    fn div(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<isize> for IPoint {
    fn div_assign(&mut self, rhs: isize) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

forward_ref_binop!(impl Div, div for IPoint, isize);
forward_ref_op_assign!(impl DivAssign, div_assign for IPoint, isize);

impl Modulo<isize> for IPoint {
    type Output = IPoint;

    fn modulo(self, rhs: isize) -> Self::Output {
        IPoint {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl ModuloAssign<isize> for IPoint {
    fn modulo_assign(&mut self, rhs: isize) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

forward_ref_binop!(impl Modulo, modulo for IPoint, isize);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for IPoint, isize);

impl Mul<IPoint> for IPoint {
    type Output = IPoint;

    fn mul(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<IPoint> for IPoint {
    fn mul_assign(&mut self, rhs: IPoint) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

forward_ref_binop!(impl Mul, mul for IPoint, IPoint);
forward_ref_op_assign!(impl MulAssign, mul_assign for IPoint, IPoint);

impl Div<IPoint> for IPoint {
    type Output = IPoint;

    fn div(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign<IPoint> for IPoint {
    fn div_assign(&mut self, rhs: IPoint) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

forward_ref_binop!(impl Div, div for IPoint, IPoint);
forward_ref_op_assign!(impl DivAssign, div_assign for IPoint, IPoint);

impl Modulo<IPoint> for IPoint {
    type Output = IPoint;

    fn modulo(self, rhs: IPoint) -> Self::Output {
        IPoint {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl ModuloAssign<IPoint> for IPoint {
    fn modulo_assign(&mut self, rhs: IPoint) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

forward_ref_binop!(impl Modulo, modulo for IPoint, IPoint);
forward_ref_op_assign!(impl ModuloAssign, modulo_assign for IPoint, IPoint);

impl ModuloPositive<IPoint> for IPoint {
    type Output = IPoint;

    fn modulo_positive(self, rhs: IPoint) -> Self::Output {
        let mut x = self.x % rhs.x;
        let mut y = self.y % rhs.y;

        if x < 0 {
            x += rhs.x;
        }

        if y < 0 {
            y += rhs.y;
        }

        IPoint { x, y }
    }
}

impl ModuloPositiveAssign<IPoint> for IPoint {
    fn module_positive_assign(&mut self, rhs: IPoint) {
        self.x %= rhs.x;
        self.y %= rhs.y;

        if self.x < 0 {
            self.x += rhs.x;
        }

        if self.y < 0 {
            self.y += rhs.y;
        }
    }
}

forward_ref_binop!(impl ModuloPositive, modulo_positive for IPoint, IPoint);
forward_ref_op_assign!(impl ModuloPositiveAssign, module_positive_assign for IPoint, IPoint);

impl ModuloPositive<isize> for IPoint {
    type Output = IPoint;

    fn modulo_positive(self, rhs: isize) -> Self::Output {
        let mut x = self.x % rhs;
        let mut y = self.y % rhs;

        if x < 0 {
            x += rhs;
        }

        if y < 0 {
            y += rhs;
        }

        IPoint { x, y }
    }
}

impl ModuloPositiveAssign<isize> for IPoint {
    fn module_positive_assign(&mut self, rhs: isize) {
        self.x %= rhs;
        self.y %= rhs;

        if self.x < 0 {
            self.x += rhs;
        }

        if self.y < 0 {
            self.y += rhs;
        }
    }
}

forward_ref_binop!(impl ModuloPositive, modulo_positive for IPoint, isize);
forward_ref_op_assign!(impl ModuloPositiveAssign, module_positive_assign for IPoint, isize);

impl Absolute for IPoint {
    fn absolute(self) -> Self {
        IPoint {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Sum for IPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(IPoint::new(0, 0), |acc, item| acc + item)
    }
}
