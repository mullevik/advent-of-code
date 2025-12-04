const DIAL_SIZE: i32 = 100;
const START: i32 = 50;

pub fn p1(input: &str) -> i32 {
    let directions = parse_directions(input);
    let mut positions = vec![50];

    for d in directions.iter() {
        let last_position: &i32 = positions.last().unwrap();
        positions.push((last_position + d).rem_euclid(100));
    }

    positions.iter().filter(|p| **p == 0).count() as i32
}

pub fn p2(input: &str) -> i32 {
    let directions = parse_directions(input);

    let mut zero_pass_count = 0;
    let mut position = START;

    for d in directions.iter() {
        for _ in 0..(d.abs()) {
            if *d > 0 {
                position = (position + 1).rem_euclid(DIAL_SIZE);
            } else {
                position = (position - 1).rem_euclid(DIAL_SIZE);
            }
            if position == 0 {
                zero_pass_count += 1;
            }
        }
    }
    zero_pass_count
}

fn parse_directions(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|x| {
            if x.starts_with("R") {
                x.replace("R", "").parse::<i32>().unwrap()
            } else {
                -x.replace("L", "").parse::<i32>().unwrap()
            }
        })
        .collect::<Vec<_>>()
}

mod tests {
    use std::fs;

    use crate::day_01::{p1, p2};

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("inputs/01.example").unwrap();
        assert_eq!(p1(&input), 3);
    }
    #[test]
    fn test_p2() {
        let input = fs::read_to_string("inputs/01.example").unwrap();
        assert_eq!(p2(&input), 6);
    }
}
