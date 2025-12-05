use std::{ops::RangeInclusive, usize};

pub fn p1(input: &str) -> i32 {
    let (ranges, fruits) = parse(input);

    fruits
        .iter()
        .filter(|f| ranges.iter().any(|r| r.contains(*f)))
        .count() as i32
}

pub fn p2(input: &str) -> i64 {
    let (ranges, _) = parse(input);

    let mut edge_points = vec![];
    for r in ranges.iter() {
        edge_points.push((r.start(), 0));
        edge_points.push((r.end(), 1));
    }

    edge_points.sort();
    let mut n_opened = 0;

    let mut final_ranges: Vec<RangeInclusive<usize>> = vec![];
    let mut start: usize = 0;
    for (v, f) in edge_points.iter() {
        if *f == 0 {
            n_opened += 1;

            if n_opened == 1 {
                start = **v
            }
        } else {
            n_opened -= 1;

            if n_opened == 0 {
                final_ranges.push(start..=**v);
            }
        }
    }

    final_ranges
        .iter()
        .map(|r| ((*r.end() + 1) as i64) - (*r.start() as i64))
        .sum()
}

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, fruits) = input.split_once("\n\n").unwrap();

    (
        ranges
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|l| parse_range(l))
            .collect::<Vec<_>>(),
        fruits
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<_>>(),
    )
}

fn parse_range(input: &str) -> RangeInclusive<usize> {
    let (lhs, rhs) = input.split_once("-").unwrap();

    lhs.parse::<usize>().unwrap()..=rhs.parse::<usize>().unwrap()
}

mod tests {
    use std::fs;

    use crate::day_05::{p1, p2};

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("inputs/05.example").unwrap();
        assert_eq!(p1(&input), 3);
    }
    #[test]
    fn test_p2() {
        let input = fs::read_to_string("inputs/05.example").unwrap();
        assert_eq!(p2(&input), 14);
    }
}
