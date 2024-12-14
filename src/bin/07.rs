use advent_of_code::parse::Parsable;
use std::iter::from_fn;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Equation {
    result: u64,
    inputs: Vec<u64>,
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

fn solvable(eq: &Equation, rem: u64, idx: usize) -> bool {
    if idx == 0 {
        return rem == 0;
    }
    let idx = idx - 1;
    let next = eq.inputs[idx];

    (rem % next == 0 && solvable(eq, rem / next, idx))
        || (rem >= next && solvable(eq, rem - next, idx))
}

/*
fn possible(eq: &Equation, acc: u64, rest: &[u64]) -> bool {
    if rest.is_empty() {
        return acc == eq.result;
    }

    possible(eq, acc + rest[0], &rest[1..]) || possible(eq, acc * rest[0], &rest[1..])
}
*/

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);

    equations
        .iter()
        .filter_map(|eq| {
            if solvable(&eq, eq.result, eq.inputs.len()) {
            //if possible(eq, 0, &eq.inputs) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

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
        assert_eq!(result, /*Some(4364915411363)*/ None);
    }
}
