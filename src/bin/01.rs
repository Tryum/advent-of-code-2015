advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let input = input.trim();

    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected character !"),
        }
    }
    Some(floor)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.trim();

    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Unexpected character !"),
        }
        if floor < 0 {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_cases = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];
        for (input, result) in test_cases {
            assert_eq!(part_one(input), Some(result))
        }
    }

    #[test]
    fn test_part_two() {
        let test_cases = [(")", 1), ("()())", 5)];
        for (input, result) in test_cases {
            assert_eq!(part_two(input), Some(result))
        }
    }
}
