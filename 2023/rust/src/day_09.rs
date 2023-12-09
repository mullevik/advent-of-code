

fn compute_difference(sequence: &Vec<i64>) -> Vec<i64> {
    sequence
    .iter()
    .zip(sequence.iter().skip(1))
    .map(|(a, b)| b - a)
    .collect::<Vec<i64>>()
}

fn compute_next_value(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().all(|x| *x == 0) {
        return 0
    }
    let differences = compute_difference(&sequence);
    let last = sequence.last().unwrap();
    compute_next_value(&differences) + last
}

pub fn first_part(input: &str) -> i64 {
    input.split("\n")
    .filter(|l| !l.trim().is_empty())
    .map(|l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>())
    .map(|x| compute_next_value(&x))
    .sum()
}

pub fn second_part(input: &str) -> i64 {
    input.split("\n")
    .filter(|l| !l.trim().is_empty())
    .map(
        |l| l.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
    )
    .map(|x| {
        let mut tmp = x.clone();
        tmp.reverse();
        tmp
    })
    .map(|x| compute_next_value(&x))
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_09::{first_part, second_part, compute_difference, compute_next_value};
    

    #[test]
    fn test_differences() {
        assert_eq!(
            compute_difference(&vec![10, 13, 16, 21, 30, 45]),
            vec![3, 3, 5, 9, 15],
        )
    }

    #[test]
    fn test_last() {
        assert_eq!(
            compute_next_value(&vec![0, 0, 0, 0]),
            0,
        );
        assert_eq!(
            compute_next_value(&vec![0, 3, 6, 9, 12, 15]),
            18
        );
        assert_eq!(
            compute_next_value(&vec![1, 3, 6, 10, 15, 21]),
            28
        );
        assert_eq!(
            compute_next_value(&vec![10, 13, 16, 21, 30, 45]),
            68
        )
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/09_example_1.txt")), 114);
        assert_eq!(second_part(include_str!("inputs/09_example_1.txt")), 2);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/09.secret")), 1930746032);
        assert_eq!(second_part(include_str!("inputs/09.secret")), 1154);
    }
}