use std::collections::HashSet;

advent_of_code::solution!(3);

fn update(c: char, x: &mut i32, y: &mut i32) {
    match c {
        '^' => {
            *y -= 1;
        }
        'v' => {
            *y += 1;
        }
        '<' => {
            *x -= 1;
        }
        '>' => {
            *x += 1;
        }
        _ => {}
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut x = 0;
    let mut y = 0;

    let mut places = HashSet::new();
    places.insert((x, y));

    for c in input.chars() {
        update(c, &mut x, &mut y);
        places.insert((x, y));
    }
    Some(places.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut santa_x = 0;
    let mut santa_y = 0;

    let mut bot_x = 0;
    let mut bot_y = 0;

    let mut places = HashSet::new();
    places.insert((santa_x, santa_y));

    let mut input_iter = input.chars();
    while let Some(c) = input_iter.next() {
        update(c, &mut santa_x, &mut santa_y);

        if let Some(c) = input_iter.next() {
            update(c, &mut bot_x, &mut bot_y);
        } else {
            break;
        }
        places.insert((santa_x, santa_y));
        places.insert((bot_x, bot_y));
    }
    Some(places.len() as u64)
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
        assert_eq!(result, Some(11));
    }
}
