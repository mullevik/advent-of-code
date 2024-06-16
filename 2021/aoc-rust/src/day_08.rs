use core::panic;
use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const N_SEGMENTS: usize = 7;
const ALL_CHARS: [char; N_SEGMENTS] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

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

pub fn second_part(input: &str) -> i64 {
    parse(input)
        .par_iter()
        .map(|p| {
            let solution = solve_csp(p);

            p.output_patterns
                .iter()
                .map(|pattern| map_digit(&solution, pattern))
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i64>()
                .unwrap()
        })
        .sum::<i64>()
}

struct Problem {
    signal_patterns: Vec<HashSet<char>>,
    output_patterns: Vec<HashSet<char>>,
}

type SegmentCandidates = [HashSet<char>; N_SEGMENTS];
type SegmentConstraints = [Vec<char>; N_SEGMENTS];

fn solve_csp(problem: &Problem) -> [char; N_SEGMENTS] {
    let candidates = build_initial_candidates(problem);

    let constraints = build_constraints(&candidates);

    let empty_candidates: Vec<char> = vec![];
    let possible_solution = backtrack(&constraints, 0, &empty_candidates, problem);
    if let Some(solution) = possible_solution {
        [
            solution[0],
            solution[1],
            solution[2],
            solution[3],
            solution[4],
            solution[5],
            solution[6],
        ]
    } else {
        panic!("Unsolvable")
    }
}

fn backtrack(
    constraints: &SegmentConstraints,
    segment_idx: usize,
    candidates: &[char],
    problem: &Problem,
) -> Option<Vec<char>> {
    if candidates.len() == N_SEGMENTS
        && problem
            .signal_patterns
            .iter()
            .all(|p| try_map_digit(candidates, p).is_some())
        && problem
            .output_patterns
            .iter()
            .all(|p| try_map_digit(candidates, p).is_some())
    {
        return Some(candidates.to_vec());
    }

    if segment_idx >= N_SEGMENTS {
        return None;
    }

    for idx in 0..constraints[segment_idx].len() {
        let new_candidate = constraints[segment_idx][idx];

        if candidates.contains(&new_candidate) {
            continue;
        }

        let mut new_candidates = candidates.to_vec();

        new_candidates.push(new_candidate);
        let possible_solution = backtrack(constraints, segment_idx + 1, &new_candidates, problem);

        if let Some(solution) = possible_solution {
            return Some(solution);
        };
    }
    None
}

fn try_map_digit(solution: &[char], pattern: &HashSet<char>) -> Option<i32> {
    match [
        pattern.contains(&solution[0]),
        pattern.contains(&solution[1]),
        pattern.contains(&solution[2]),
        pattern.contains(&solution[3]),
        pattern.contains(&solution[4]),
        pattern.contains(&solution[5]),
        pattern.contains(&solution[6]),
    ] {
        [false, true, true, false, false, false, false] => Some(1),
        [true, true, false, true, true, false, true] => Some(2),
        [true, true, true, true, false, false, true] => Some(3),
        [false, true, true, false, false, true, true] => Some(4),
        [true, false, true, true, false, true, true] => Some(5),
        [true, false, true, true, true, true, true] => Some(6),
        [true, true, true, false, false, false, false] => Some(7),
        [true, true, true, true, true, true, true] => Some(8),
        [true, true, true, true, false, true, true] => Some(9),
        [true, true, true, true, true, true, false] => Some(0),
        _ => None,
    }
}

fn map_digit(solution: &[char], pattern: &HashSet<char>) -> i32 {
    if let Some(number) = try_map_digit(solution, pattern) {
        number
    } else {
        println!("solution {:?} pattern {:?}", solution, pattern);
        panic!("Wrong digit")
    }
}

fn build_constraints(candidates: &SegmentCandidates) -> SegmentConstraints {
    let mut constraints = [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for (i, c) in candidates.iter().enumerate() {
        constraints[i] = c.iter().copied().collect::<Vec<char>>();
    }
    constraints
}

fn build_initial_candidates(problem: &Problem) -> SegmentCandidates {
    let unknown_digits = ALL_CHARS.iter().cloned().collect::<HashSet<_>>();
    let mut segment_candidates = [
        unknown_digits.clone(),
        unknown_digits.clone(),
        unknown_digits.clone(),
        unknown_digits.clone(),
        unknown_digits.clone(),
        unknown_digits.clone(),
        unknown_digits.clone(),
    ];

    fn remove_candidates(
        indices: &[usize],
        segment_candidates: &mut SegmentCandidates,
        pattern: &HashSet<char>,
    ) {
        for i in indices.iter() {
            segment_candidates[*i] = (&segment_candidates[*i] - pattern).clone();
        }
    }

    fn set_candidates(
        indices: &[usize],
        segment_candidates: &mut SegmentCandidates,
        pattern: &HashSet<char>,
    ) {
        for i in indices.iter() {
            segment_candidates[*i] = segment_candidates[*i]
                .intersection(pattern)
                .cloned()
                .collect();
        }
    }

    problem.signal_patterns.iter().for_each(|pattern| {
        if pattern.len() == 2 {
            set_candidates(&[1, 2], &mut segment_candidates, pattern);
            remove_candidates(&[0, 3, 4, 5, 6], &mut segment_candidates, pattern);
        } else if pattern.len() == 4 {
            set_candidates(&[1, 2, 5, 6], &mut segment_candidates, pattern);
            remove_candidates(&[0, 3, 4], &mut segment_candidates, pattern);
        } else if pattern.len() == 3 {
            set_candidates(&[0, 1, 2], &mut segment_candidates, pattern);
            remove_candidates(&[3, 4, 5, 6], &mut segment_candidates, pattern);
        } else if pattern.len() == 7 {
            set_candidates(&[0, 1, 2, 3, 4, 5, 6], &mut segment_candidates, pattern);
        }
    });

    segment_candidates
}

fn parse(input: &str) -> Vec<Problem> {
    input
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (before, after) = line.split_once('|').unwrap();
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

    use std::collections::HashSet;

    use crate::day_08::build_initial_candidates;

    use super::{first_part, parse, second_part, solve_csp};

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
        assert_eq!(first_part(include_str!("../inputs/08.in")), 493);
    }

    #[test]
    fn test_initial_state() {
        let input: &'static str =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let problems = parse(input);

        // gotcha: you cant just do: let problem = parse(input).first().unwrap();

        assert_eq!(
            build_initial_candidates(problems.first().unwrap()),
            [
                ['d'].iter().cloned().collect::<HashSet<_>>(),
                ['a', 'b'].iter().cloned().collect::<HashSet<_>>(),
                ['a', 'b'].iter().cloned().collect::<HashSet<_>>(),
                ['c', 'g'].iter().cloned().collect::<HashSet<_>>(),
                ['c', 'g'].iter().cloned().collect::<HashSet<_>>(),
                ['e', 'f'].iter().cloned().collect::<HashSet<_>>(),
                ['e', 'f'].iter().cloned().collect::<HashSet<_>>(),
            ]
        );
    }

    #[test]
    fn test_solution() {
        let input: &'static str =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let problems = parse(input);

        assert_eq!(
            solve_csp(problems.first().unwrap())
                .iter()
                .collect::<HashSet<_>>(),
            ['d', 'b', 'a', 'c', 'g', 'e', 'f']
                .iter()
                .collect::<HashSet<_>>()
        )
    }

    #[test]
    fn test_solution_part() {
        let input: &'static str =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

        assert_eq!(second_part(input), 5353);
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(include_str!("../inputs/08_example")), 61229);
    }
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/08.in")), 1010460);
    }
}
