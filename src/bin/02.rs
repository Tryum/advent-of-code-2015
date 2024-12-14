use std::cmp::min;

use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"(?m)(\d*)x(\d*)x(\d*)").unwrap();

    let mut result = 0;
    for [l, w, h] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<u64>().unwrap()))
    {
        let a = l * w;
        let b = w * h;
        let c = h * l;

        let s = min(min(a, b), c);
        result += 2 * (a + b + c) + s;
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(?m)(\d*)x(\d*)x(\d*)").unwrap();

    let mut result = 0;
    for [l, w, h] in re
        .captures_iter(input)
        .map(|c| c.extract().1.map(|x| x.parse::<u64>().unwrap()))
    {
        let mut sides = [l, w, h];
        sides.sort();

        let perimeter = 2 * sides[0] + 2 * sides[1];
        let volume = sides[0] * sides[1] * sides[2];

        result += perimeter + volume;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
