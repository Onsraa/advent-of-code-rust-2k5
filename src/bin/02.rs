advent_of_code::solution!(2);

fn parse_line(line: &str) -> Option<Vec<(u64, u64)>> {
    let result: Vec<(u64, u64)> = line
        .split(',')
        .filter_map(|s| {
            let (left, right) = s.split_once('-')?;
            let left = left.parse::<u64>().ok()?;
            let right = right.parse::<u64>().ok()?;
            Some((left, right))
        })
        .collect();

    Some(result)
}

fn compare_id(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len % 2 != 0 || len == 1 {
        return false;
    }

    let mut left: usize = 0;
    let mut right: usize = len / 2;

    while right < len {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right += 1;
    }
    true
}

pub fn is_invalid_id(s: &str) -> bool {

    let bytes = s.as_bytes();
    let len = bytes.len();

    for pattern_len in 1..=(len / 2) {
        if len % pattern_len == 0 {
            let mut is_pattern = true;

            for i in 0..(len - pattern_len) {
                if bytes[i] != bytes[i + pattern_len] {
                    is_pattern = false;
                    break
                }
            }

            if is_pattern {
                return true
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    if let Some(ids) = parse_line(input) {
        for (left_id, right_id) in ids {
            for i in left_id..=right_id {
                let s = i.to_string();
                //println!("check {s:?}");
                if compare_id(&s) {
                    sum += i
                }
            }
        }
    };
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    if let Some(ids) = parse_line(input) {
        for (left_id, right_id) in ids {
            for i in left_id..=right_id {
                let s = i.to_string();
                if is_invalid_id(&s) {
                    //println!("add : {i}");
                    sum += i
                }
            }
        }
    };
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31000881061));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46769308485));
    }
}
