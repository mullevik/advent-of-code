use std::{char, collections::HashSet};

pub fn first_part(input: &str) -> i32 {
    let problems = parse(input);
    -1
}

struct Problem {
    signal_patterns: Vec<HashSet<char>>,
    output_value: HashSet<char>,
}

fn parse(input: &str) -> Vec<Problem> {
    input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (before, after) = line.split_once("|").unwrap();
            let before_patterns = before
                .split_whitespace()
                .map(|part| part.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let after_pattern = after.chars().collect::<HashSet<_>>();

            Problem {
                signal_patterns: before_patterns,
                output_value: after_pattern,
            }
        })
        .collect::<Vec<Problem>>()
}

mod test_day_08 {

    use super::{parse, first_part};


    #[test]
    fn test_problem_parsing() {
        let problems = parse(include_str!("../inputs/08_example"));
        assert_eq!(problems.len(), 10);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("../inputs/08_example")), 493);
    }
}
