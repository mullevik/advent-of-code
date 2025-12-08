// purely functional challange - I appologize to the readers

pub fn p1(input: &str) -> i64 {
    count_splits(
        initial_beams(get_start(input), get_width(input))
            .iter()
            .map(|x| *x > 0)
            .collect::<Vec<_>>(),
        input
            .split("\n")
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>(),
        0,
    )
}

pub fn p2(input: &str) -> i64 {
    count_beams(
        initial_beams(get_start(input), get_width(input)),
        input
            .split("\n")
            .filter(|line| !line.trim().is_empty())
            .map(|l| l.to_string())
            .collect::<Vec<String>>(),
    )
}

fn count_splits(beams: Vec<bool>, lines: Vec<String>, n_splits: i64) -> i64 {
    match lines.first() {
        Some(line) => count_splits(
            get_next_beams(&beams, line),
            lines[1..lines.len()].iter().cloned().collect::<Vec<_>>(),
            if line.contains('^') {
                get_next_beams(&beams, line)
                    .iter()
                    .zip(beams)
                    .map(|(new_b, b)| if b && !new_b { 1 } else { 0 })
                    .sum::<i64>()
                    + n_splits
            } else {
                n_splits
            },
        ),
        None => n_splits,
    }
}

fn get_next_beams(beams: &Vec<bool>, line: &String) -> Vec<bool> {
    (0..beams.len())
        .map(|b| {
            if has_splitter_at(line, b as i32) {
                false
            } else if has_splitter_at(line, b as i32 - 1) || has_splitter_at(line, b as i32 + 1) {
                true
            } else {
                beams[b]
            }
        })
        .collect::<Vec<_>>()
}

fn count_beams(beams: Vec<i64>, lines: Vec<String>) -> i64 {
    match lines.first() {
        Some(line) => count_beams(
            (0..beams.len())
                .map(|b| {
                    if has_splitter_at(line, b as i32) {
                        0
                    } else {
                        beams[b]
                            + {
                                if has_splitter_at(line, b as i32 - 1) {
                                    beams[b - 1]
                                } else {
                                    0
                                }
                            }
                            + {
                                if has_splitter_at(line, b as i32 + 1) {
                                    beams[b + 1]
                                } else {
                                    0
                                }
                            }
                    }
                })
                .collect::<Vec<_>>(),
            lines[1..lines.len()].iter().cloned().collect::<Vec<_>>(),
        ),
        None => beams.iter().sum(),
    }
}

fn has_splitter_at(line: &String, pos: i32) -> bool {
    if pos < 0 {
        false
    } else if pos >= line.len() as i32 {
        false
    } else {
        line.chars().nth(pos as usize).unwrap() == '^'
    }
}

fn initial_beams(start_position: usize, width: usize) -> Vec<i64> {
    (0..width)
        .map(|x| if x == start_position { 1 } else { 0 })
        .collect::<Vec<_>>()
}

fn get_width(input: &str) -> usize {
    input.split("\n").next().unwrap().len()
}

fn get_start(input: &str) -> usize {
    input
        .split("\n")
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap()
}

mod tests {
    use std::fs;

    use crate::day_07::{p1, p2};

    #[test]
    fn test_p1() {
        assert_eq!(p1(&fs::read_to_string("inputs/07.example").unwrap()), 21);
    }
    #[test]
    fn test_p2() {
        assert_eq!(p2(&fs::read_to_string("inputs/07.example").unwrap()), 40);
    }
}
