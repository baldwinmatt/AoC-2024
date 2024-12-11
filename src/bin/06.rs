use std::collections::HashSet;

use advent_of_code::point::Point;
use advent_of_code::pointmap::{PointMap, Direction};

advent_of_code::solution!(6);


#[derive(PartialEq, Eq)]
enum Square {
    Empty,
    Obstacle,
}

fn parse(input: &str) -> (PointMap<Square>, Point) {

    let mut vec = Vec::new();

    let mut height = 0;
    let mut start = Point::new(0, 0);

    for l in input.lines() {
        for (x, b) in l.bytes().enumerate() {
            match b {
                b'.' => vec.push(Square::Empty),
                b'#' => vec.push(Square::Obstacle),
                b'^' => {
                    start.x = x;
                    start.y = height;
                    vec.push(Square::Empty);
                }
                _ => panic!("Invalid character: {}", b as char),
            }
        }
        height += 1;
    }

    (PointMap::from_vec(vec, height), start)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start) = parse(input);

    let mut visited = PointMap::from_vec(vec![false; map.width * map.height], map.height);
    let mut pos = start;
    let mut count = 1;
    let mut dir = Direction::North;
    visited[pos] = true;

    while let Some(next) = map.step(pos, dir) {
        if map[next] == Square::Obstacle {
            dir = dir.rotate_clockwise();
            continue;
        } else if !visited[next] {
            visited[next] = true;
            count += 1;
        }

        pos = next;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start) = parse(input);

    let mut visited = PointMap::from_vec(vec![[false; 4]; map.width * map.height], map.height);
    let mut pos = start;
    let mut dir = Direction::North;
    let mut loops = HashSet::new();
    visited[pos][0] = true;

    while let Some(next) = map.step(pos, dir) {
        if map[next] == Square::Obstacle {
            dir = dir.rotate_clockwise();
            continue;
        } else if visited[next].iter().all(|seen| !seen) {
            let mut visited_dup = PointMap::from_vec(vec![[false; 4]; map.width * map.height], map.height);
            let mut dir_dup = dir.rotate_clockwise();
            let mut pos_dup = pos;

            while let Some(next_dup) = map.step(pos_dup, dir_dup) {
                if map[next_dup] == Square::Obstacle || next_dup == next {
                    dir_dup = dir_dup.rotate_clockwise();
                    continue;
                } else if visited_dup[next_dup][dir_dup as usize]
                    || visited[next_dup][dir_dup as usize] {
                    loops.insert(next);
                    break;
                }

                visited_dup[next_dup][dir_dup as usize] = true;
                pos_dup = next_dup;
            }
        }

        visited[next][dir as usize] = true;
        pos = next;
    }

    Some(loops.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
