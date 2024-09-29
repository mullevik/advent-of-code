use rustc_hash::{FxHashMap, FxHashSet};

pub fn first_part(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            if let Points::Invalid(x) = parse_line(line) {
                x
            } else {
                0
            }
        })
        .sum()
}
pub fn second_part(input: &str) -> i64 {
    let mut scores = input
        .lines()
        .filter_map(|line| match parse_line(line) {
            Points::Invalid(_) => None,
            Points::Complete(x) => Some(x),
        })
        .collect::<Vec<i64>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn get_validity_map() -> FxHashMap<char, (char, i64)> {
    [
        (')', ('(', 3)),
        (']', ('[', 57)),
        ('}', ('{', 1197)),
        ('>', ('<', 25137)),
    ]
    .into_iter()
    .collect()
}

#[derive(PartialEq, Eq, Debug)]
enum Points {
    Invalid(i64),
    Complete(i64),
}

fn parse_line(line: &str) -> Points {
    let mut stack = Vec::with_capacity(100);
    let valid_map = get_validity_map();

    for c in line.chars() {
        if let Some((matching_char, points)) = valid_map.get(&c) {
            if stack.last().unwrap() != matching_char {
                return Points::Invalid(points.to_owned());
            } else {
                stack.pop();
            }
        } else {
            stack.push(c);
        }
    }

    let complete_map = get_autocomplete_map();
    let mut score = 0;
    for c in stack.iter().rev() {
        score *= 5;
        score += complete_map.get(c).unwrap().1;
    }

    Points::Complete(score)
}

fn get_autocomplete_map() -> FxHashMap<char, (char, i64)> {
    [
        ('(', (')', 1)),
        ('[', (']', 2)),
        ('{', ('}', 3)),
        ('<', ('>', 4)),
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests_day_10 {
    use super::*;

    const EXAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";
    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(EXAMPLE_INPUT), 26397);
    }
    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/10.in")), 323691);
    }

    #[test]
    fn test_scoring() {
        assert_eq!(
            parse_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            Points::Complete(294)
        );
    }

    #[test]
    fn test_example_second_part() {
        assert_eq!(second_part(EXAMPLE_INPUT), 288957);
    }
    
    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/10.in")), 2858785164);
    }
}
