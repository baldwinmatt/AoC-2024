use advent_of_code::parse::Parsable;
use std::iter::from_fn;

advent_of_code::solution!(5);

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = Vec::new();

    // use an iterator so we can maintain position across rules and jobs
    let mut line_iter = input.lines().into_iter();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        let mut bytes = line.bytes();
        let x: usize = bytes.next_number().unwrap();
        let y = bytes.next_number().unwrap();
        if x >= rules.len() {
            rules.resize(x + 1, Vec::new());
        }
        rules[x].push(y);
    }

    let mut jobs = Vec::new();
    while let Some(line) = line_iter.next() {
        let mut bytes = line.bytes();
        jobs.push(from_fn(|| bytes.next_number()).collect());
    }

    (rules, jobs)
}

fn check(job: &[u32], rules: &Vec<Vec<u32>>) -> Option<u32> {
    for i in 0..job.len() - 1 {
        if !rules[job[i] as usize].contains(&job[i + 1]) {
            return None;
        }
    }
    Some(job[(job.len() - 1) / 2])
}

fn correct(job: &mut Vec<u32>, rules: &Vec<Vec<u32>>) -> u32 {
    job.sort_by(|a, b| {
        if rules[*a as usize].contains(b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    job[(job.len() - 1) / 2]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, jobs) = parse(input);

    jobs.iter()
        .filter_map(|job| check(job, &rules))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut jobs) = parse(input);

    jobs.iter_mut()
        .filter(|job| check(job, &rules).is_none())
        .map(|mut job| correct(&mut job, &rules))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
