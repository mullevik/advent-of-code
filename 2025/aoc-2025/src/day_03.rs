const RADIX: i64 = 10;

pub fn p1(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|x| highest_joltage_general(x, 2))
        .sum()
}

pub fn p2(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|x| highest_joltage_general(x, 12))
        .sum()
}

fn highest_joltage_general(x: &str, n: usize) -> i64 {
    let nums = x
        .chars()
        .map(|c| c.to_digit(RADIX as u32).unwrap())
        .collect::<Vec<_>>();

    let mut choices = vec![0; n];

    let mut n_choices = 0;
    let mut start = 0;

    for _ in 0..n {
        let end = nums.len() - (n - n_choices) + 1;
        let maximum = nums[start..end].iter().max().unwrap();

        let maximum_pos = nums[start..end].iter().position(|v| v == maximum).unwrap();

        println!(
            "s {} e {} m {} mp {} c {}",
            start, end, maximum, maximum_pos, n_choices
        );

        choices[n_choices] = *maximum;
        n_choices += 1;
        start += maximum_pos + 1;
    }

    choices
        .iter()
        .enumerate()
        .map(|(i, c)| RADIX.pow((n - (i + 1)) as u32) * (*c as i64))
        .sum()
}

mod tests {
    use std::fs;

    use crate::day_03::{highest_joltage_general, p1, p2};

    #[test]
    fn test_p1() {
        let input_03 = fs::read_to_string("inputs/03.example").unwrap();
        assert_eq!(p1(&input_03), 357);
    }

    #[test]
    fn test_highest_joltage() {
        assert_eq!(highest_joltage_general("811111111111119", 2), 89);
        assert_eq!(highest_joltage_general("987654321111111", 12), 987654321111);
        assert_eq!(highest_joltage_general("818181911112111", 12), 888911112111);
    }
    #[test]
    fn test_p2() {
        let input_03 = fs::read_to_string("inputs/03.example").unwrap();
        assert_eq!(p2(&input_03), 3121910778619);
    }
}
