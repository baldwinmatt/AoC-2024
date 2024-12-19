use advent_of_code::{pointmap::{Direction, PointMap}, point::Point};

advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Wall,
    Box,
}

fn parse(input: &str) -> (PointMap<Cell>, Point, Vec<Direction>) {
    // The input is delimeted by a newline, where the first section is a map.
    // In the map, the following characters are used:
    //   . = Empty
    //   # = Wall
    //   O = Box
    //   @ = Start
    let mut vec = Vec::new();
    let mut start = Point::new(0, 0);
    let mut height = 0;
    let mut lines = input.lines();

    // first parse the map into a PointMap from a Vec of Cells
    while let Some(line) = lines.next() {
        if line.is_empty() { break; }
        for (x, c) in line.bytes().enumerate() {
            match c {
                b'.' => vec.push(Cell::Empty),
                b'#' => vec.push(Cell::Wall),
                b'O' => vec.push(Cell::Box),
                b'@' => {
                    vec.push(Cell::Empty);
                    start = Point::new(x, height);
                },
                _ => panic!("Invalid character in input"),
            }
        }
        height+=1;
    }
    let map = PointMap::from_vec(vec, height);
    let mut dirs = Vec::new();
    // Next parse the directions, where
    // ^ is north, > is east, v is south and < is west.
    while let Some(line) = lines.next() {
        for c in line.bytes() {
            match c {
                b'^' => dirs.push(Direction::North),
                b'>' => dirs.push(Direction::East),
                b'v' => dirs.push(Direction::South),
                b'<' => dirs.push(Direction::West),
                _ => panic!("Invalid character in input"),
            }
        }
    }

    (map, start, dirs)
}

fn move_boxes(map: &mut PointMap<Cell>, pos: Point, dir: Direction) -> Point {

    if let Some(dst) = map.step(pos, dir) {
        // We can move into an empty cell
        if map[dst] == Cell::Empty {
            return dst;
        }
        else if map[dst] == Cell::Box {
            // We need to move the box to move into this cell
            let mut cur = dst;
            while let Some(p) = map.step(cur, dir) {
                // An empty cell, we can move all boxes to here, so advance our position
                if map[p] == Cell::Empty {
                    map[dst] = Cell::Empty;
                    map[p] = Cell::Box;
                    return dst;
                } // You have hit a wall!
                else if map[p] == Cell::Wall {
                    return pos;
                }
                cur = p;
            }
        }
    }

    pos
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut map, start, dirs) = parse(input);
    let mut pos = start;
    for dir in dirs {
        pos = move_boxes(&mut map, pos, dir);
    }

    map.points()
        .filter_map(|p| {
            if map[p] == Cell::Box {
                Some(p.y * 100 + p.x)
            } else {
                None
            }
        })
        .sum::<usize>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
