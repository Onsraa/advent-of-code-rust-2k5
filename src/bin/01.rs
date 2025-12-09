advent_of_code::solution!(1);

fn parse_line(line: &str) -> Option<(char, &str)> {
    let mut chars = line.chars();
    let first_char = chars.next()?;
    let rest = &line[first_char.len_utf8()..];
    Some((first_char, rest))
}

pub fn part_one(input: &str) -> Option<u64> {

    let mut dial: i32 = 50;
    let mut counter: u64 = 0;

    for line in input.lines() {
        if let Some((direction, number)) = parse_line(line) {
            if let Ok(value) = number.parse::<u32>() {
                match direction {
                    'L' => {
                        dial = ((dial - value as i32) % 100 + 100) % 100;
                    },
                    'R' => {
                        dial = ((dial + value as i32) % 100 + 100) % 100
                    },
                    _ => {return None}
                }
                if dial == 0 {
                    counter += 1
                }
            }
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {

    let mut big_dial = 1_000_050;
    let mut counter: u64 = 0;

    for line in input.lines() {
        if let Some((direction, number)) = parse_line(line) {
            if let Ok(value) = number.parse::<u32>() {

                match direction {
                    'L' => {
                        let new_dial = big_dial - value;
                        counter += ((big_dial - 1) / 100 - (new_dial - 1) / 100) as u64;
                        big_dial = new_dial;
                    },
                    'R' => {
                        let new_dial = big_dial + value;
                        counter += (new_dial / 100 - big_dial / 100) as u64;
                        big_dial = new_dial;
                    },
                    _ => {}
                }
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1064));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6122));
    }
}
