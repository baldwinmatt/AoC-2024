use anyhow::*;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        left.push(row[0]);
        right.push(row[1]);
    }
    left.sort();
    right.sort();
    Ok((left, right))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input).unwrap();

    let total = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| (r - l).abs())
        .sum::<i32>();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input).unwrap();

    let total: usize = left
        .iter()
        .map(|&l| right.iter().filter(|&r| *r == l).count() * (l as usize))
        .sum();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
