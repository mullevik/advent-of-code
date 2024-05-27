use rustc_hash::FxHashMap;

const LIFETIME_DAYS: i32 = 7;

pub fn first_part(input: &str) -> i64 {
    solve(&parse(input), 80)
}

pub fn second_part(input: &str) -> i64 {
    solve(&parse(input), 256)
}

fn solve(states: &[i32], n_days: i32) -> i64 {
    let mut memoization: FxHashMap<i32, i64> = FxHashMap::default();

    states
        .iter()
        .map(|state| fish_count(n_days + (LIFETIME_DAYS + 2) - (state + 1), &mut memoization))
        .sum::<i64>()
}

fn fish_count(days: i32, memoization: &mut FxHashMap<i32, i64>) -> i64 {
    let memoized = memoization.get(&days);

    if let Some(memoized_value) = memoized {
        *memoized_value
    } else {
        let new_value = (0..=(days - (LIFETIME_DAYS + 2)))
            .rev()
            .step_by(LIFETIME_DAYS as usize)
            .map(|days_left| fish_count(days_left, memoization))
            .sum::<i64>()
            + 1;

        memoization.insert(days, new_value);
        new_value
    }
}

fn parse(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|part| part.trim())
        .map(|num: &str| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

mod tests {

    use super::{first_part, fish_count, second_part};
    use rustc_hash::FxHashMap;

    #[test]
    fn test_fish_count() {
        assert_eq!(fish_count(0, &mut FxHashMap::default()), 1);
        assert_eq!(fish_count(5, &mut FxHashMap::default()), 1);
        assert_eq!(fish_count(6, &mut FxHashMap::default()), 1);
        assert_eq!(fish_count(7, &mut FxHashMap::default()), 1);
        assert_eq!(fish_count(8, &mut FxHashMap::default()), 1);
        assert_eq!(fish_count(9, &mut FxHashMap::default()), 2);
    }

    #[test]
    fn test_range() {
        let actual = (0..10).step_by(5).rev().collect::<Vec<_>>();
        assert_eq!(actual, vec![5, 0]);
        assert_eq!((0..0).collect::<Vec<_>>(), vec![]);
        assert_eq!((0..=0).collect::<Vec<_>>(), vec![0]);
        assert_eq!((1..0).rev().collect::<Vec<_>>(), vec![]);
        assert_eq!((1..10).rev().step_by(5).collect::<Vec<_>>(), vec![9, 4]);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("../inputs/06_example")), 5934);
        assert_eq!(
            second_part(include_str!("../inputs/06_example")),
            26984457539
        );
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/06")), 387413);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/06")), 1738377086345);
    }
}
