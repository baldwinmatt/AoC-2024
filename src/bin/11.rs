use std::collections::{LinkedList, HashMap};

use advent_of_code::parse::Parseable;

advent_of_code::solution!(11);

fn parse(input: &str) -> LinkedList<u64> {
    let mut l = LinkedList::new();
    let mut bytes = input.bytes();
    while let Some(n) = bytes.next_number() {
        l.push_back(n);
    }
    l
}

fn blink(stone: u64, times: u32, cache: &mut HashMap<u64, u64>) -> u64 {
    let key = stone * 100 + (times as u64);
    if let Some(cached) = cache.get(&key) {
        return *cached;
    }

    let len = if stone < 10 { 1 } else { stone.ilog10() as u32 + 1};
    let sum = if times == 1 {
        return if len % 2 == 0 {
            2
        } else {
            1
        }
    } else {
        if stone == 0 {
            blink(1, times - 1, cache)
        } else {
            if len % 2 == 0 {
                let mut pow = 1;
                for _ in 0..len / 2 {
                    pow *= 10;
                }

                blink(stone / pow, times - 1, cache)
                    + blink(stone % pow, times - 1, cache)
            } else {
                blink(stone * 2024, times - 1, cache)
            }
        }
    };

    cache.insert(key, sum);

    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = parse(input);
    let mut cache = HashMap::new();
    Some (stones.iter().fold(0, |acc, stone| {
        acc + blink(*stone, 25, &mut cache)
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = parse(input);
    let mut cache = HashMap::new();
    Some (stones.iter().fold(0, |acc, stone| {
        acc + blink(*stone, 75, &mut cache)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
