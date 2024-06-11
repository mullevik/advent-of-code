use std::{
    cmp::{max, min},
    i64,
};

use crate::utils::sign;

pub fn first_part(input: &str) -> i32 {
    let crab_positions = parse_input(input);
    // solve_iteratively(&crab_positions, difference_distance)
    solve_with_gradient_descent(&crab_positions, difference_distance)
}
pub fn second_part(input: &str) -> i32 {
    let crab_positions = parse_input(input);
    solve_with_gradient_descent(&crab_positions, arithmetic_distance)
    // solve_iteratively(&crab_positions, arithmetic_distance)
}

fn solve_iteratively(crab_positions: &[i32], distance_fn: fn(i32, i32) -> i32) -> i32 {
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();

    (min_pos..=max_pos)
        .map(|current_pos| count_fuel(current_pos, crab_positions, distance_fn))
        .min()
        .unwrap()
}

fn solve_with_gradient_descent(crab_positions: &[i32], distance_fn: fn(i32, i32) -> i32) -> i32 {
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();

    let mut current_position = 0;
    let mut step_size = max_pos - min_pos;
    let mut total_min = i32::MAX;
    let mut last_total_min_set = 0;
    loop {
        let f_0 = count_fuel(current_position, crab_positions, distance_fn);
        let f_1 = count_fuel(current_position + 1, crab_positions, distance_fn);
        let grad = f_0 - f_1;

        if f_0 < total_min || f_1 < total_min {
            total_min = min(f_0, f_1);
        } else {
            last_total_min_set += 1;
        }

        current_position = current_position + (sign(grad)) * step_size;
        step_size = max(step_size / 2, 1);

        if last_total_min_set > 10 {
            break;
        }
    }
    total_min
}

fn count_fuel(
    current_position: i32,
    crab_positions: &[i32],
    distance_fn: fn(i32, i32) -> i32,
) -> i32 {
    crab_positions
        .iter()
        .map(|crab_position| distance_fn(*crab_position, current_position))
        .sum::<i32>()
}

fn difference_distance(a: i32, b: i32) -> i32 {
    a.abs_diff(b) as i32
}

fn arithmetic_distance(a: i32, b: i32) -> i32 {
    let diff = a.abs_diff(b);

    if diff == 0 {
        0
    } else {
        (diff * (1 + diff) / 2) as i32
    }
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

mod test_day_07 {
    use super::{first_part, second_part};

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/07_example")), 37);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/07")), 343468);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/07_example")), 168);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/07")), 96086265);
    }
}
