

pub fn first_part(input: &str) -> i32 {
    let crab_positions = parse_input(input);
    // solve_iteratively(&crab_positions, difference_distance)
    solve_with_bisect(&crab_positions, difference_distance)
}
pub fn second_part(input: &str) -> i32 {
    let crab_positions = parse_input(input);
    solve_with_bisect(&crab_positions, arithmetic_distance)
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

fn solve_with_bisect(crab_positions: &[i32], distance_fn: fn(i32, i32) -> i32) -> i32 {
    let mut a = *crab_positions.iter().min().unwrap();
    let mut b = *crab_positions.iter().max().unwrap();

    let mut f_a = count_fuel(a, crab_positions, distance_fn);
    let mut f_b = count_fuel(b, crab_positions, distance_fn);

    while a != b {
        if f_a > f_b {
            a = (a + 1) + (a.abs_diff(b) / 2) as i32;
            f_a = count_fuel(a, crab_positions, distance_fn);
        } else {
            b = (b - 1) - (a.abs_diff(b) / 2) as i32;
            f_b = count_fuel(b, crab_positions, distance_fn);
        }
    }
    f_a
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
        assert_eq!(first_part(include_str!("../inputs/07.in")), 343468);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/07_example")), 168);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/07.in")), 96086265);
    }
}
