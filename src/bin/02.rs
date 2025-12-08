advent_of_code::solution!(2);

fn parse_line(line: &str) -> Option<Vec<(u64, u64)>> {

    let result: Vec<(u64, u64)> = line.split(',').filter_map(|s| {
        let (left, right) = s.split_once('-')?;
        let left = left.parse::<u64>().ok()?;
        let right = right.parse::<u64>().ok()?;
        Some((left, right))
    }).collect();

    Some(result)
}

fn compare_id(s: &str) -> bool {

    let len = s.len();

    if len % 2 != 0 { return false }

    let mut left: usize = 0;
    let mut right: usize = len / 2;

    while right < len {
        if s.chars().nth(left) != s.chars().nth(right) { return false }
        left += 1;
        right += 1;
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    if let Some(ids) = parse_line(input){
        for (left_id, right_id) in ids {
            for i in left_id..=right_id {
                let s = i.to_string();
                if compare_id(&s) { sum += i }
            }
        }
    };
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
