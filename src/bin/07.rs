use advent_of_code::parse::Parsable;
use std::{iter::from_fn, fmt::Display};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u64,
    inputs: Vec<u64>,
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.result, self.inputs.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "))
    }
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut bytes = line.bytes();

            Equation {
                result: bytes.next_number().unwrap(),
                inputs: from_fn(|| bytes.next_number()).collect(),
            }
        })
        .collect()
}

fn solvable(eq: &Equation, rem: u64, idx: usize) -> Option<u64> {
    if idx == 0 {
        return if rem == 0 { Some(eq.result) } else { None }
    }
    let idx = idx - 1;
    let next = eq.inputs[idx];

    if (rem % next == 0 && solvable(eq, rem / next, idx).is_some())
        || (rem >= next && solvable(eq, rem - next, idx).is_some()) {
        return Some(eq.result);
    }

    None
}

fn process_three_operators(target: u64, components: &[u64], idx: usize, acc: u64) -> bool {

    if acc > target {
        return false;
    }

    if idx == components.len() {
        return acc == target;
    }

    let next = components[idx];
    let mut pow = 1;
    while pow < next {
        pow *= 10;
    }

    process_three_operators(target, components, idx + 1, acc + next) ||
        process_three_operators(target, components, idx + 1, acc * next) ||
        process_three_operators(target, components, idx + 1, acc * pow + next)
}

fn process_two_operators(target: u64, components: &[u64], idx: usize, acc: u64) -> bool {

    if acc > target {
        return false;
    }

    if idx == components.len() {
        return acc == target;
    }

    return process_two_operators(target, components, idx + 1, acc + components[idx]) ||
        process_two_operators(target, components, idx + 1, acc * components[idx]);
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);

    equations
        .iter()
        .filter_map(|eq| {
            solvable(&eq, eq.result, eq.inputs.len())
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);

    equations
        .iter()
        .filter_map(|eq| {
            if process_two_operators(eq.result, &eq.inputs, 0, 0) ||
                process_three_operators(eq.result, &eq.inputs, 0, 0) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
