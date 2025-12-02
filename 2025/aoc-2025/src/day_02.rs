pub fn solve_part_one(input: &str) -> i64 {
    input
        .split(",")
        .filter(|x| x.trim().len() > 0)
        .map(|x| find_missing_in_range_half(x))
        .flatten()
        .sum()
}

pub fn solve_part_two(input: &str) -> i64 {
    input
        .split(",")
        .filter(|x| x.trim().len() > 0)
        .map(|x| find_missing_in_range(x))
        .flatten()
        .sum()
}

fn find_missing_in_range_half(range: &str) -> Vec<i64> {
    let (left, right) = range.split_once("-").unwrap();
    let left_num = left.trim().parse::<i64>().unwrap();
    let right_num = right.trim().parse::<i64>().unwrap();
    (left_num..=right_num)
        .filter(|x| is_invalid_half(x))
        .collect::<Vec<_>>()
}

fn is_invalid_half(x: &i64) -> bool {
    let x_str = format!("{}", x);
    if x_str.len() % 2 == 1 {
        false
    } else {
        is_invalid(&x_str, x_str.len() / 2)
    }
}

fn find_missing_in_range(range: &str) -> Vec<i64> {
    let (left, right) = range.split_once("-").unwrap();
    let left_num = left.trim().parse::<i64>().unwrap();
    let right_num = right.trim().parse::<i64>().unwrap();
    (left_num..=right_num)
        .filter(|x| is_invalid_any(x))
        .collect::<Vec<_>>()
}

fn is_invalid_any(x: &i64) -> bool {
    let x_str = format!("{}", x);

    (1..=x_str.len() / 2).any(|a| is_invalid(&x_str, a))
}

fn is_invalid(x_str: &String, size: usize) -> bool {
    let mut parts: Vec<String> = vec![];
    let n = x_str.len() / size;
    for i in 0..n {
        if i == n - 1 {
            parts.push(x_str[i * size..].to_string());
        } else {
            parts.push(x_str[i * size..(i * size) + size].to_string());
        }
    }
    let first_part = parts.first().unwrap();
    parts.iter().all(|p| p == first_part)
}

mod tests {
    use std::fs;

    use crate::day_02::{is_invalid_any, is_invalid_half, solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        let input_02 = fs::read_to_string("inputs/02_example").unwrap();
        assert_eq!(solve_part_one(input_02.as_str()), 1227775554);
    }
    #[test]
    fn test_part_two() {
        let input_02 = fs::read_to_string("inputs/02_example").unwrap();
        assert_eq!(solve_part_two(input_02.as_str()), 4174379265);
    }

    #[test]
    fn test_is_invalid_half() {
        assert!(!is_invalid_half(&10101));
        assert!(is_invalid_half(&11));
        assert!(!is_invalid_half(&12));
        assert!(is_invalid_half(&446446))
    }
    #[test]
    fn test_is_invalid() {
        assert!(is_invalid_any(&111));
        assert!(is_invalid_any(&824824824));
        assert!(!is_invalid_any(&12));
        assert!(is_invalid_any(&565656))
    }
}
