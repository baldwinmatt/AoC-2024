use std::collections::{HashSet, HashMap};

use advent_of_code::{pointmap::PointMap, point::Point};

advent_of_code::solution!(10);

fn parse(input: &str) -> PointMap<u8>{
    let mut vec = Vec::new();
    let mut height = 0;
    for line in input.lines() {
        for b in line.bytes() {
            vec.push(b);
        }
        height += 1;
    }
    PointMap::from_vec(vec, height)
}

fn count_trail_heads(map: &PointMap<u8>, start: Point, unique: bool) -> u64 {
    let mut visited = HashSet::<Point>::new();
    let mut queue = Vec::new();
    let mut trail_ends = HashMap::<Point, u64>::new();

    queue.push(start);
    while let Some(p) = queue.pop() {
        if unique && visited.contains(&p) {
            continue;
        }
        visited.insert(p);

        if *map.at(p) == b'9' {
            *trail_ends.entry(p).or_default() += 1;
        }
        let target_height = map.at(p) + 1;
        for n in map.neighbors(p) {
            if n.is_none() {
                continue;
            }

            let n = n.unwrap();
            if *map.at(n) != target_height  || (visited.contains(&n) && unique) {
                continue;
            }

            queue.push(n);
        }
    }

    trail_ends.values().into_iter().sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse(input);
    let mut result = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point::new(x, y);
            let c = map.at(p);
            if *c == b'0' {
                result += count_trail_heads(&map, p, true)
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse(input);
    let mut result = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let p = Point::new(x, y);
            let c = map.at(p);
            if *c == b'0' {
                result += count_trail_heads(&map, p, false)
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
