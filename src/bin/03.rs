use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<u32>().unwrap();
            a * b
        })
        .reduce(|acc, e| acc + e)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();

    let r = re
        .find_iter(input)
        .map(|m| m.as_str())
        .fold((0, true), |(sum, enabled), inst| {
            let new_enabled = match inst {
                "do()" => true,
                "don't()" => false,
                _ => enabled,
            };

            let new_sum = sum
                + if enabled && inst.starts_with("mul") {
                    let re = Regex::new(r"mul\((?<a>[0-9]{1, 3}),(?<b>[0-9]{1, 3})\)").unwrap();
                    let m = re.captures(inst).unwrap();
                    let a = m.name("a").unwrap().as_str().parse::<u32>().unwrap();
                    let b = m.name("b").unwrap().as_str().parse::<u32>().unwrap();
                    a * b
                } else {
                    0
                };
            (new_sum, new_enabled)
        })
        .0;
    Some(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
