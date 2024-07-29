use std::{char, collections::HashMap};

pub fn first_part(input: &str) -> i64 {
    solve(input, 10)
}

pub fn second_part(input: &str) -> i64 {
    solve(input, 40)
}

fn solve(input: &str, n_iterations: i32) -> i64 {
    let (polymer, rules) = parse(input);

    let mut element_counts = count_elements(&polymer);
    let mut memoization_table = HashMap::new();

    for window in polymer.chars().collect::<Vec<_>>().windows(2) {
        let counts = count_polymers(
            window[0],
            window[1],
            n_iterations,
            &rules,
            &mut memoization_table,
        );

        element_counts = merge_counts(&element_counts, &counts);
    }

    element_counts.iter().map(|kv| kv.1).max().unwrap()
        - element_counts.iter().map(|kv| kv.1).min().unwrap()
}

type Counter = HashMap<char, i64>;

fn count_elements(polymer: &String) -> Counter {
    let mut counter = HashMap::default();

    for element in polymer.chars() {
        let count = counter.entry(element).or_insert(0);
        *count += 1;
    }

    counter
}

fn count_polymers(
    left: char,
    right: char,
    ttl: i32,
    rules: &HashMap<(char, char), char>,
    memoization_table: &mut HashMap<(char, char, i32), Counter>,
) -> Counter {
    if let Some(val) = memoization_table.get(&(left, right, ttl)) {
        return val.to_owned();
    }

    if ttl == 0 {
        Counter::new()
    } else {
        let middle = rules.get(&(left, right)).unwrap();

        let mut merged = merge_counts(
            &count_polymers(left, *middle, ttl - 1, rules, memoization_table),
            &count_polymers(*middle, right, ttl - 1, rules, memoization_table),
        );

        let middle_val = merged.entry(*middle).or_insert(0);
        *middle_val += 1;

        memoization_table.insert((left, right, ttl), merged.clone());
        merged
    }
}

fn merge_counts(a: &Counter, b: &Counter) -> Counter {
    let mut merged = HashMap::new();

    for (k, v) in a.iter().chain(b.iter()) {
        let val_ref = merged.entry(k.to_owned()).or_insert(0);
        *val_ref += v;
    }

    merged
}

fn parse(input: &str) -> (String, HashMap<(char, char), char>) {
    let (polymer, rules) = input.split_once("\n\n").unwrap();

    (polymer.to_string(), parse_rules(rules))
}

fn parse_rules(input: &str) -> HashMap<(char, char), char> {
    input
        .lines()
        .map(|row| {
            let (from, to) = row.split_once("->").unwrap();
            let from_chars = from.trim().chars().collect::<Vec<char>>();
            (
                (from_chars[0], from_chars[1]),
                to.trim().to_string().chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>()
}

mod tests_day_14 {
    use crate::{day_14::first_part, day_14::parse, day_14::second_part};

    #[test]
    fn test_parse() {
        let (polymer, rules) = parse(include_str!("../inputs/14_example"));

        assert_eq!(polymer, "NNCB".to_string());
        assert_eq!(rules.len(), 16);
        assert_eq!(rules.get(&('C', 'N')), Some('C').as_ref());
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/14_example")), 1588);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/14.in")), 2851);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(
            second_part(include_str!("../inputs/14_example")),
            2188189693529
        );
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/14.in")), 10002813279337);
    }
}
