const RADIX: i64 = 10;

pub fn solve_part_one(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|x| highest_joltage_general(x, 2))
        .sum()
}

pub fn solve_part_two(input: &str) -> i64 {
    input
        .split("\n")
        .filter(|x| x.trim().len() > 0)
        .map(|x| highest_joltage_general(x, 12))
        .sum()
}

fn highest_joltage(x: &str) -> i32 {
    let mut m = 0;

    for i in 0..(x.len() - 1) {
        for j in i + 1..x.len() {
            let val = format!("{}{}", x.chars().nth(i).unwrap(), x.chars().nth(j).unwrap())
                .parse::<i32>()
                .unwrap();
            if val > m {
                m = val;
            }
        }
    }
    m
}

fn highest_joltage_general(x: &str, n: usize) -> i64 {
    let nums = x
        .chars()
        .map(|c| c.to_digit(RADIX as u32).unwrap())
        .collect::<Vec<_>>();

    let mut choice_map = vec![false; x.len()];

    let mut n_choices = 0;

    let mut starts = vec![0];

    while n_choices < n {
        let mut start = *starts.last().unwrap();
        while choice_map[start..nums.len()]
            .iter()
            .all(|is_chosen| *is_chosen)
        {
            starts.pop();
            start = *starts.last().unwrap();
        }

        let m = nums[start..nums.len()]
            .iter()
            .zip(choice_map[start..nums.len()].iter())
            .filter(|(_, is_chosen)| !*is_chosen)
            .map(|(v, _)| v)
            .max()
            .unwrap();

        let m_pos = nums[start..nums.len()]
            .iter()
            .zip(choice_map[start..nums.len()].iter())
            .position(|(v, is_chosen)| !is_chosen && v == m)
            .unwrap();

        choice_map[start + m_pos] = true;
        n_choices += 1;
        starts.push(start + m_pos + 1);
    }

    let choices = nums
        .iter()
        .zip(choice_map.iter())
        .filter(|(_, is_chosen)| **is_chosen)
        .map(|(n, _)| n)
        .collect::<Vec<_>>();

    choices
        .iter()
        .enumerate()
        .map(|(i, c)| RADIX.pow((n - (i + 1)) as u32) * (**c as i64))
        .sum()
}

mod tests {
    use std::fs;

    use crate::day_03::{highest_joltage, highest_joltage_general, solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        let input_03 = fs::read_to_string("inputs/03.example").unwrap();
        assert_eq!(solve_part_one(&input_03), 357);
    }

    #[test]
    fn test_highest_joltage() {
        assert_eq!(highest_joltage_general("811111111111119", 2), 89);
        assert_eq!(highest_joltage_general("987654321111111", 12), 987654321111);
        assert_eq!(highest_joltage_general("818181911112111", 12), 888911112111);
    }
    #[test]
    fn test_part_two() {
        let input_03 = fs::read_to_string("inputs/03.example").unwrap();
        assert_eq!(solve_part_two(&input_03), 3121910778619);
    }
}
