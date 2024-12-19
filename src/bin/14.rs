use std::{ffi::FromVecWithNulError, iter::from_fn};

use advent_of_code::{point::{ipoint::IPoint, traits::{ModuloPositive, Absolute}}, parse::Parseable, math::solve_linear_diophantine};

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    pos: IPoint,
    velocity: IPoint,
}

impl Robot {
    fn siumulate(&mut self, time: isize, dims: IPoint) {
        let pos = (self.pos + self.velocity * time).modulo_positive(dims);
        self.pos = pos;
    }
}

fn parse(input: &str) -> Vec<Robot> {
    let mut bytes = input.bytes();
    from_fn(|| bytes.next_number().zip(bytes.next_number()))
        .map(|(pos, velocity)| Robot { pos, velocity })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {

    let width = 101;
    let height = 103;

    Some(solve_one(input, width, height))
}

fn solve_one(input: &str, width: isize, height: isize) -> usize {
    let mut robots = parse(input);

    let quad_width = width / 2;
    let quad_height = height / 2;

    let mut quads = vec![0; 4];
    let dims = IPoint::new(width, height);

    for robot in robots.iter_mut() {
        robot.siumulate(100, dims);

        if robot.pos.x == quad_width || robot.pos.y == quad_height {
            continue;
        }

        let quad = if robot.pos.x > quad_width { 0b10 } else { 0 }
            | if robot.pos.y > quad_height { 1 } else { 0 };

        quads[quad] += 1;
    }

    quads.iter().product()
}

fn solve_two(input: &str, width: isize, height: isize) -> usize {
    let mut robots = parse(input);

    let dims = IPoint::new(width, height);
    let mut x_start = None;
    let mut y_start = None;
    let mut i = 1;

    while x_start.is_none() || y_start.is_none() {
        for robot in robots.iter_mut() {
            robot.siumulate(1, dims);
        }

        let var = variance(&robots);

        if var.x < 8000 {
            x_start = Some(i);
        }
        if var.y < 8000 {
            y_start = Some(i);
        }
        i += 1;
    }

    if let Some((x, y)) = x_start.zip(y_start) {
        if let Some((x, _)) = solve_linear_diophantine(width, -height, y - x) {
            return (x * width + y) as usize;
        }
    }

    0
}

fn variance(robots: &Vec<Robot>) -> IPoint {
    let average = robots.iter().map(|r| r.pos).sum::<IPoint>() / robots.len() as isize;
    robots.iter().map(|r| (average - r.pos).absolute()).sum()
}

pub fn part_two(input: &str) -> Option<usize> {

    let width = 101;
    let height = 103;

    Some(solve_two(input, width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        //assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, None);
    }
}
