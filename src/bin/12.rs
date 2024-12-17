use std::collections::{VecDeque, HashSet};

use advent_of_code::{pointmap::PointMap, point::{Point, ipoint::IPoint}};

advent_of_code::solution!(12);

fn parse(input: &str) -> PointMap<char> {
    let mut vec = Vec::new();
    let mut height = 0;
    input.lines().for_each(|line| {
        line.bytes().for_each(|b| {
            vec.push(b as char);
        });
        height += 1;
    });
    PointMap::from_vec(vec, height)
}

struct Region {
    plant: char,
    cells: Vec<Point>,
}

impl Region {
    fn area(&self) -> usize {
        self.cells.len()
    }

    fn perimiter(&self, map: &PointMap<char>) -> usize {
        let mut perimiter = 0;
        for cell in &self.cells {
            let neighbors = map.neighbors(*cell);
            for neighbor in neighbors {
                if let Some(n) = neighbor {
                    if *map.at(n) != self.plant {
                        perimiter += 1;
                    }
                } else {
                    perimiter += 1;
                }
            }
        }
        perimiter
    }

    fn edge_count(&self, map: &PointMap<char>) -> usize {
        let mut count = 0;
        for cell in &self.cells {
            count += self.corner_count(cell, map);
        }
        count
    }

    fn corner_count(&self, cell: &Point, map: &PointMap<char>) -> usize {
        let mut count = 0;
        let mut dx = -1;
        let mut dy = 0;

        for _ in 0..4 {
            let d1 = IPoint::new(dx, dy);
            let d2 = IPoint::new(dy, -dx);

            let c1 = IPoint::from_point(*cell) + d1;
            let c2 = IPoint::from_point(*cell) + d2;
            let c3 = IPoint::from_point(*cell) + d1 + d2;
            let a1 = map.ati(c1);
            let a2 = map.ati(c2);
            let a3 = map.ati(c3);
            if !self.is_cell_plant(a1) && !self.is_cell_plant(a2) {
                count += 1;
            } else if self.is_cell_plant(a1) && self.is_cell_plant(a2) &&
                !self.is_cell_plant(a3){
                count += 1;
            }

            (dx, dy) = (dy, -dx);
        }

        count
    }

    fn is_cell_plant(&self, ati: Option<&char>) -> bool{
        if let Some(c) = ati {
            if *c == self.plant {
                return true;
            }
        }
        false
    }
}

struct RegionMapper {
    map: PointMap<char>,
    regions: Vec<Region>,

    visited: HashSet<Point>,
}

impl RegionMapper {
    fn new(map: PointMap<char>) -> Self {
        let mut mapper = Self { map, regions: Vec::new(), visited: HashSet::new() };

        for x in 0..mapper.map.width {
            for y in 0..mapper.map.height {
                let point = Point { x, y };
                if mapper.visited.contains(&point) {
                    continue;
                }
                let region = mapper.map_region(point);
                mapper.regions.push(region);
            }
        }
        mapper
    }

    fn map_region(&mut self, point: Point) -> Region {
        let mut region = Region { plant: *self.map.at(point), cells: Vec::new() };
        let mut queue = VecDeque::new();
        queue.push_back(point);

        while let Some(point) = queue.pop_front() {
            if self.visited.contains(&point) {
                continue;
            }
            self.visited.insert(point);
            region.cells.push(point);

            let neighbors = self.map.neighbors(point);
            for neighbor in neighbors {
                if let Some(n) = neighbor {
                    if !self.visited.contains(&n) && *self.map.at(n) == region.plant {
                        queue.push_back(n);
                    }
                }
            }
        }
        region
    }
}

pub fn part_one(input: &str) -> Option<usize> {

    let mapper = RegionMapper::new(parse(input));

    mapper.regions.iter().map(|region| region.area() * region.perimiter(&mapper.map)).sum::<usize>().into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mapper = RegionMapper::new(parse(input));

    mapper.regions.iter().map(|region| region.area() * region.edge_count(&mapper.map)).sum::<usize>().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
