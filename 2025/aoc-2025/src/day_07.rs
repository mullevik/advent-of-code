pub fn p1(input: &str) -> i32 {
    let (start, tachyons) = parse_input(input);

    let mut beams = vec![false; tachyons.first().unwrap().len()];
    beams[start] = true;

    let mut n_splits = 0;
    for tachyon_row in tachyons.iter() {
        for (x, t) in tachyon_row.iter().enumerate() {
            if beams[x] && *t {
                beams[x - 1] = true;
                beams[x + 1] = true;
                beams[x] = false;
                n_splits += 1;
            }
        }
    }

    n_splits
}

pub fn p2(input: &str) -> i64 {
    let (start, tachyons) = parse_input(input);
    let n_cols = tachyons.first().unwrap().len();
    let mut beams = vec![0; n_cols];
    beams[start] = 1;

    for tachyon_row in tachyons.iter() {
        for (x, t) in tachyon_row.iter().enumerate() {
            if beams[x] > 0 && *t {
                beams[x - 1] = beams[x - 1] + beams[x];
                beams[x + 1] = beams[x + 1] + beams[x];
                beams[x] = 0;
            }
        }
    }
    beams.iter().sum()
}

fn parse_input(input: &str) -> (usize, Vec<Vec<bool>>) {
    let non_empty_lines = input
        .split("\n")
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>();

    let dim_x = non_empty_lines.first().unwrap().len();
    let mut start = 0;
    let mut tachyons = vec![];
    non_empty_lines.iter().for_each(|line| {
        for (x, c) in line.chars().enumerate() {
            let mut tachyons_row = vec![false; dim_x];

            if c == 'S' {
                start = x;
            } else if c == '^' {
                tachyons_row[x] = true;
            }

            if tachyons_row.iter().any(|x| *x) {
                tachyons.push(tachyons_row);
            }
        }
    });

    (start, tachyons)
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
        let input = fs::read_to_string("inputs/07.example").unwrap();
        assert_eq!(p2(&input), 40);
    }
}
