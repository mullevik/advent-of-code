use regex::Regex;
use rustc_hash::FxHashMap;

use crate::utils::sign;


pub fn first_part(input: &str) -> i32 {
    let pairs = parse_pairs(input);
    let non_diagonal_pairs = pairs
        .iter()
        .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .cloned()
        .collect::<Vec<_>>();
    solve_with_pairs(&non_diagonal_pairs)
}

pub fn second_part(input: &str) -> i32 {
    let pairs = parse_pairs(input);
    solve_with_pairs(&pairs)
}

fn solve_with_pairs(pairs: &[((i32, i32), (i32, i32))]) -> i32 {
    let mut counter: FxHashMap<(i32, i32), i32> = FxHashMap::default();
    for pair in pairs.iter() {
        let line = generate_line(pair.0, pair.1);
        for p in line {
            *counter.entry(p).or_insert(0) += 1;
        }
    }
    counter.values().filter(|count| **count > 1).count() as i32
}

struct LineIterator {
    direction: (i32, i32),
    position: (i32, i32),
    end_position: (i32, i32),
}

impl LineIterator {
    fn new(source: (i32, i32), destination: (i32, i32), direction: (i32, i32)) -> Self {
        LineIterator {
            direction: direction,
            position: source,
            end_position: destination,
        }
    }
}

impl Iterator for LineIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let prev_position = self.position;
        let next_position = (
            prev_position.0 + self.direction.0,
            prev_position.1 + self.direction.1,
        );

        if self.direction == (0, 0) {
            None
        } else if prev_position == self.end_position {
            self.direction = (0, 0);
            Some(prev_position)
        } else {
            self.position = next_position;
            Some(prev_position)
        }
    }
}

fn generate_line(a: (i32, i32), b: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    let dx = -sign(a.0 - b.0);
    let dy = -sign(a.1 - b.1);
    LineIterator::new(a, b, (dx, dy))
}

fn parse_pairs(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let re = Regex::new(r"(\d+),(\d+)\s->\s(\d+),(\d+)").unwrap();
            let captures = re.captures(line).unwrap();
            let source = (
                captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let destination = (
                captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );
            (source, destination)
        })
        .collect::<Vec<_>>()
}

mod tests {
    use super::generate_line;

    #[test]
    fn test_line_build() {
        assert_eq!(
            generate_line((0, 0), (3, 0)).collect::<Vec<_>>(),
            vec![(0, 0), (1, 0), (2, 0), (3, 0)]
        );

        assert_eq!(generate_line((0, 0), (10, 0)).collect::<Vec<_>>().len(), 11);
        assert_eq!(generate_line((0, 0), (0, 10)).collect::<Vec<_>>().len(), 11);
        assert_eq!(
            generate_line((0, 0), (-10, 0)).collect::<Vec<_>>().len(),
            11
        );
        assert_eq!(
            generate_line((0, 0), (0, -10)).collect::<Vec<_>>().len(),
            11
        );
        assert_eq!(
            generate_line((0, 0), (10, 10)).collect::<Vec<_>>().len(),
            11
        );
        assert_eq!(
            generate_line((0, 0), (10, -10)).collect::<Vec<_>>().len(),
            11
        );
        assert_eq!(
            generate_line((0, 0), (-10, -10)).collect::<Vec<_>>().len(),
            11
        );
        assert_eq!(
            generate_line((0, 0), (-10, 10)).collect::<Vec<_>>().len(),
            11
        );
    }
    // add code here
}
