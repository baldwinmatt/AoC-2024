use advent_of_code::parse::Parseable;

advent_of_code::solution!(13);

// Input is formatted as:
//
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
//
// This defines a single ClawMachine.  There are multiple machines, delimted by a blank New Line

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn parse(input: &str) -> Vec<ClawMachine> {
    let mut machines = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let button_a = line;
        let button_b = lines.next().unwrap();
        let prize = lines.next().unwrap();
        let mut button_a = button_a.bytes();
        let button_a = (
            button_a.next_number().unwrap(),
            button_a.next_number().unwrap(),
        );
        let mut button_b = button_b.bytes();
        let button_b = (
            button_b.next_number().unwrap(),
            button_b.next_number().unwrap(),
        );
        let mut prize = prize.bytes();
        let prize = (prize.next_number().unwrap(), prize.next_number().unwrap());
        machines.push(ClawMachine {
            button_a,
            button_b,
            prize,
        });
    }
    machines
}

// solve a pair of linear equations such that:
// given a = button_a.0, b = button_a.1, c = button_b.0, d = button_b.1, x = prize.0, y = prize.1
// where A and B are integers from 0..100
// find A, B such that:
// a * A + b * A = x
// c * B + d * B = y
fn solve(cm: &ClawMachine, offset: i64) -> Option<u64> {
    let x = cm.prize.0 + offset;
    let y = cm.prize.1 + offset;

    let a = cm.button_a.0;
    let c = cm.button_a.1;
    let b = cm.button_b.0;
    let d = cm.button_b.1;

    let left = a * y - c * x;
    let right = a * d - b * c;
    if left % right == 0 {
        let tb = left / right;

        let left = x - b * tb;
        let right = a;
        if left % right == 0 {
            let ta = left / right;

            return Some((3 * ta + tb) as u64);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);

    machines
        .iter()
        .filter_map(|cm| solve(cm, 0))
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);

    machines
        .iter()
        .filter(|cm| solve(cm, 0).is_none())
        .filter_map(|cm| solve(cm, 10000000000000))
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
