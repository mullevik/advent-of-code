use std::{char, collections::HashSet};

pub fn first_part(input: &str) -> i32 {
    let problems = parse(input);

    problems
        .iter()
        .map(|problem| {
            problem
                .output_patterns
                .iter()
                .filter(|pattern| {
                    (pattern.len() == 2)
                        || (pattern.len() == 4)
                        || (pattern.len() == 3)
                        || (pattern.len() == 7)
                })
                .count() as i32
        })
        .sum::<i32>()
}

struct Problem {
    signal_patterns: Vec<HashSet<char>>,
    output_patterns: Vec<HashSet<char>>,
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
            let after_patterns = after
                .split_whitespace()
                .map(|part| part.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            Problem {
                signal_patterns: before_patterns,
                output_patterns: after_patterns,
            }
        })
        .collect::<Vec<Problem>>()
}

mod test_day_08 {
    use super::{first_part, parse};

    #[test]
    fn test_problem_parsing() {
        let problems = parse(include_str!("../inputs/08_example"));
        assert_eq!(problems.len(), 10);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("../inputs/08_example")), 26);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/08")), 493);
    }
}
