use std::ops::{Index, IndexMut};

use crate::point::{Point, ipoint::IPoint};

#[derive(Debug, Clone)]
pub struct PointMap<T> {
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }

    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::North => Self::West,
            Direction::East => Self::North,
            Direction::South => Self::East,
            Direction::West => Self::South,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Self::South,
            Direction::East => Self::West,
            Direction::South => Self::North,
            Direction::West => Self::East,
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as usize
    }
}

impl<T> PointMap<T> {
    pub fn from_vec(vec: Vec<T>, height: usize) -> Self {
        let width = vec.len() / height;
        Self { vec, width, height }
    }

    pub fn new(vec: Vec<T>, width: usize, height: usize) -> Self {
        Self { vec, width, height }
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.vec[self.width * y..(self.width * y + self.width)]
    }

    pub fn at(&self, point: Point) -> &T {
        &self.vec[self.width * point.y + point.x]
    }

    pub fn ati(&self, point: IPoint) -> Option<&T> {
        if point.in_bounds(self.width, self.height) {
            Some(self.at(Point::from_ipoint(point)))
        } else {
            None
        }
    }

    pub fn at_mut(&mut self, point: Point) -> &mut T {
        &mut self.vec[self.width * point.y + point.x]
    }

    pub fn set(&mut self, point: Point, value: T) {
        self.vec[self.width * point.y + point.x] = value;
    }

    pub fn is_in_bounds(&mut self, point: Point) -> bool {
        point.x < self.width && point.y < self.height
    }

    pub fn is_in_boundsi(&mut self, point: IPoint) -> bool {
        point.in_bounds(self.width, self.height)
    }

    pub fn step(&self, point: Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::North => self.step_north(point),
            Direction::East => self.step_east(point),
            Direction::South => self.step_south(point),
            Direction::West => self.step_west(point),
        }
    }

    pub fn neighbors(&self, point: Point) -> [Option<Point>; 4] {
        [
            self.step_north(point),
            self.step_east(point),
            self.step_south(point),
            self.step_west(point),
        ]
    }

    pub fn step_north(&self, point: Point) -> Option<Point> {
        if point.y == 0 {
            None
        } else {
            Some(Point::new(point.x, point.y - 1))
        }
    }

    pub fn step_east(&self, point: Point) -> Option<Point> {
        if point.x == self.width - 1 {
            None
        } else {
            Some(Point::new(point.x + 1, point.y))
        }
    }
    pub fn step_south(&self, point: Point) -> Option<Point> {
        if point.y == self.height - 1 {
            None
        } else {
            Some(Point::new(point.x, point.y + 1))
        }
    }
    pub fn step_west(&self, point: Point) -> Option<Point> {
        if point.x == 0 {
            None
        } else {
            Some(Point::new(point.x - 1, point.y))
        }
    }
}

impl<T> Index<Point> for PointMap<T> {
    fn index(&self, index: Point) -> &T {
        self.at(index)
    }

    type Output = T;
}

impl<T> IndexMut<Point> for PointMap<T> {
    fn index_mut(&mut self, index: Point) -> &mut T {
        self.at_mut(index)
    }
}
