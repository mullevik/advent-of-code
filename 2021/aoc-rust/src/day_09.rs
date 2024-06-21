use crate::grid::Grid;

pub fn first_part(input: &str) -> i32 {
    let g = parse(input);

    g.iter()
        .filter(|(p, v)| g.four_neighborhood_at(p).iter().all(|n| *n > v))
        .map(|(_, v)| v + 1)
        .sum::<i32>()
}

pub fn second_part(input: &str) -> i32 {
    unimplemented!()
}

fn parse(input: &str) -> Grid<i32> {
    Grid::from_rows(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            }),
    )
    .unwrap()
}

mod tests_day_09 {

    use super::*;

    #[test]
    fn test_first_part_example() {
        assert_eq!(first_part(include_str!("../inputs/09_example")), 15);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/09.in")), -1)
    }
}
