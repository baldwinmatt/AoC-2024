advent_of_code::solution!(4);

fn parse_into_vec(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes().to_vec()
        })
        .collect::<Vec<Vec<u8>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_into_vec(input);

    let mut count = 0;

    for y in 0..map.len() {
        let r = &map[y];
        for x in 0..r.len() {
            let c = r[x];
            if c != b'X' {
                continue;
            }
            // look for XMAS in each of the 8 directions
            if x >= 3 {
                let m = r[x - 1] == b'M' && r[x - 2] == b'A' && r[x - 3] == b'S';
                if m { count += 1; }
                // back diagonal-up
                if y >= 3 {
                    let m = map[y - 1][x - 1] == b'M' &&
                        map[y - 2][x - 2] == b'A' &&
                        map[y - 3][x - 3] == b'S';

                    if m { count += 1; }
                }
                // back diagonal-down
                if y < map.len() - 3 {
                    let m = map[y + 1][x - 1] == b'M' &&
                        map[y + 2][x - 2] == b'A' &&
                        map[y + 3][x - 3] == b'S';

                    if m { count += 1; }
                }
            }
            if x < r.len() - 3 {
                let m = r[x + 1] == b'M' && r[x + 2] == b'A' && r[x + 3] == b'S';
                if m { count += 1; }
                // forward diagonal-down
                if y < map.len() - 3 {
                    let m = map[y + 1][x + 1] == b'M' &&
                        map[y + 2][x + 2] == b'A' &&
                        map[y + 3][x + 3] == b'S';

                    if m { count += 1; }
                }
                // forward diagonal-up
                if y >= 3 {
                    let m = map[y - 1][x + 1] == b'M' &&
                        map[y - 2][x + 2] == b'A' &&
                        map[y - 3][x + 3] == b'S';

                    if m { count += 1; }
                }
            }
            if y >= 3 {
                let m = map[y - 1][x] == b'M' &&
                    map[y - 2][x] == b'A' &&
                    map[y - 3][x] == b'S';

                if m { count += 1; }
            }
            if y < map.len() - 3 {
                let m = map[y + 1][x] == b'M' &&
                    map[y + 2][x] == b'A' &&
                    map[y + 3][x] == b'S';

                if m { count += 1; }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_into_vec(input);

    let mut count = 0;

    // Search for all the 'A' bytes, then see if:
    // (([y-1][x-1] == 'M' && [y + 1][x + 1] == 'S') ||
    // ([y-1][x-1] == 'S' && [y + 1][x + 1] == 'M')) &&
    // (([y+1][x-1] == 'M' && [y - 1][x + 1] == 'S') ||
    // ([y+1][x-1] == 'S' && [y - 1][x + 1] == 'M'))
    for y in 1..map.len() - 1 {
        let r = &map[y];
        for x in 1..r.len() - 1 {
            let c = r[x];
            if c != b'A' {
                continue;
            }
            if ((map[y - 1][x - 1] == b'M' && map[y + 1][x + 1] == b'S') ||
                (map[y - 1][x - 1] == b'S' && map[y + 1][x + 1] == b'M')) &&
                ((map[y + 1][x - 1] == b'M' && map[y - 1][x + 1] == b'S') ||
                (map[y + 1][x - 1] == b'S' && map[y - 1][x + 1] == b'M')) {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
