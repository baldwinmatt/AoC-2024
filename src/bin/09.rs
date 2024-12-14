use advent_of_code::parse;

advent_of_code::solution!(9);

enum Entry {
    Free { size: usize },
    File { id: usize, size: usize },
}

fn parse_input(input: &str) -> Vec<Entry> {
    let mut id = 0;

    input
        .char_indices()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let id = i / 2;
                Entry::File { id, size }
            } else {
                Entry::Free { size }
            }
        })
        .collect()
}

fn defragment(disk: &mut Vec<Entry>, mut free: usize, write_cursor: usize, clean: &mut Vec<Entry>) {
    let mut read_cursor = disk.len() - 1;

    while read_cursor > write_cursor && free > 0 {
        let block = &disk[read_cursor];
        match *block {
            Entry::File { id, size } => {
                if size <= free {
                    clean.push(Entry::File { id, size });
                    free -= size;
                    disk.remove(read_cursor);
                    read_cursor -= 1;
                } else {
                    clean.push(Entry::File { id, size: free });
                    disk[read_cursor] = Entry::File {
                        id,
                        size: size - free,
                    };
                    free = 0;
                }
            }
            Entry::Free { size: _ } => {
                read_cursor -= 1;
            }
        }
    }
}

fn checksum(memory: &[Entry]) -> usize {
    let mut position = 0;
    let mut result = 0;

    for block in memory {
        match *block {
            Entry::Free { size } => position += size,
            Entry::File { id, size } => {
                for _ in 0..size {
                    result += id * position;
                    position += 1;
                }
            }
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = parse_input(input);
    let mut clean: Vec<Entry> = Vec::new();

    let mut write_cursor = 0;
    while write_cursor < disk.len() {
        let block = &disk[write_cursor];
        match *block {
            Entry::File { id, size } => clean.push(Entry::File { id, size }),
            Entry::Free { size } => {
                defragment(&mut disk, size, write_cursor, &mut clean);
            }
        }

        write_cursor += 1;
    }

    Some(checksum(&clean))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
