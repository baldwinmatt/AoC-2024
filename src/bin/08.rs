advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use advent_of_code::{point::{ipoint::IPoint, Point}, pointmap::PointMap};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Antinode,
    Antenna(char),
}

fn parse_map(input: &str) -> PointMap<Tile> {
    let mut vec = Vec::new();
    let mut height = 0;

    for line in input.lines() {
        for c in line.bytes() {
            match c {
                b'.' => vec.push(Tile::Empty),
                _ => vec.push(Tile::Antenna(c as char)),
            }
        }
        height += 1;
    }

    PointMap::from_vec(vec, height)
}


fn parse(input: &str) -> (HashMap<u8, Vec<IPoint>>, usize, usize) {
    let mut map: HashMap<u8, Vec<IPoint>> = HashMap::new();

    let mut y = 0;
    let mut width = 0;
    for line in input.lines() {
        for (x, c) in line.bytes().enumerate() {
            if y == 0 {
                width += 1;
            }
            match c {
                b'.' => continue,
                _ =>
                    map.entry(c).or_default().push(IPoint::new(x as i32, y as i32))
            }
        }
        y+=1;
    }

    (map, width, y)
}

trait BoundedPoint {
    fn in_bounds(&self, width: usize, height: usize) -> bool;
    fn bounded_sub(&self, other: IPoint, width: usize, height: usize) -> Option<IPoint>;
    fn bounded_add(&self, other: IPoint, width: usize, height: usize) -> Option<IPoint>;
}

impl BoundedPoint for IPoint {
    fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.x < width as i32 && self.y < height as i32 && self.x >= 0 && self.y >= 0
    }

    fn bounded_add(&self, other: IPoint, width: usize, height: usize) -> Option<IPoint> {
        let p = *self + other;
        if p.in_bounds(width, height) {
            return Some(p);
        }
        None
    }

    fn bounded_sub(&self, other: IPoint, width: usize, height: usize) -> Option<IPoint> {
        let p = *self - other;
        if p.in_bounds(width, height) {
            return Some(p);
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, width, height) = parse(input);

    let mut antinodes = HashSet::new();
    for pos in map.values() {
        for pair in pos.iter().combinations(2) {
            let a1 = *pair[0];
            let a2 = *pair[1];

            let diff = a2 - a1;
            let anti1 = a2.bounded_add(diff, width, height);
            let anti2 = a1.bounded_sub(diff, width, height);

            if let Some(p) = anti1 {
                antinodes.insert(p);
            }
            if let Some(p) = anti2 {
                antinodes.insert(p);
            }
        }
    }

   Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, width, height) = parse(input);

    let mut antinodes = HashSet::new();
    for pos in map.values() {
        for pair in pos.iter().combinations(2) {
            let mut a1 = *pair[0];
            let mut a2 = *pair[1];

            let diff = a2 - a1;
            loop {
                let anti1 = a2.bounded_add(diff, width, height);
                let anti2 = a1.bounded_sub(diff, width, height);
                let mut added = false;

                if let Some(p) = anti1 {
                    a2 = p;
                    antinodes.insert(p);
                    added = true;
                }
                if let Some(p) = anti2 {
                    a1 = p;
                    antinodes.insert(p);
                    added = true;
                }
                if !added {
                    break;
                }
            }
        }
    }

    let mut m = parse_map(input);
    for ele in antinodes.iter() {
        m.set(Point::from_ipoint(*ele), Tile::Antinode);
    }

    for y in 0..m.height {
        let row = m.row(y);
        println!("{}", row.iter().map(|t| match t {
            Tile::Empty => '.',
            Tile::Antinode => '#',
            Tile::Antenna(c) => *c
        }).collect::<String>());
    }

   Some(antinodes.len())
}

/*
##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
