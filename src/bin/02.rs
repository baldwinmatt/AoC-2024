advent_of_code::solution!(2);

fn solve(row: &Vec<i32>, recurse: bool) -> bool {
    let mut safe = true;
    let inc = if row[0] > row[1] {
        1
    } else if row[0] < row[1] {
        -1
    } else {
        0
    };
    for i in 1..(row.len()) {
        let diff = (row[i - 1] - row[i]) * inc;
        safe = diff >= 1 && diff <= 3;

        if !recurse && !safe {
            break;
        } else if !safe {
            // time to get fancy

            let mut row_no_i = row.to_vec();
            row_no_i.remove(i);
            let mut row_no_i_minus_1 = row.to_vec();
            row_no_i_minus_1.remove(i - 1);

            safe = solve(&row_no_i, false) || solve(&row_no_i_minus_1, false);

            if !safe && i == 2 {
                let mut row_no_i_minus_2 = row.to_vec();
                row_no_i_minus_2.remove(i - 2);
                safe = solve(&row_no_i_minus_2, false);
            }

            break;
        }
    }

    safe
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    for line in input.lines() {
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if solve(&row, false) {
            safe_count += 1;
        }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count = 0;

    for line in input.lines() {
        let row = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        if solve(&row, true) {
            safe_count += 1;
        }
    }
    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
