use crate::grid::Grid;

pub fn first_part(input: &str) -> i32 {
    todo!();
}
pub fn second_part(input: &str) -> i32 {
    todo!();
}

fn parse(input: &str) -> Grid<i32> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .collect()
}
#[cfg(test)]
mod tests_day_15 {
    use crate::day_15::parse;

    #[test]
    fn test_parse() {
        let g = parse(include_str!("../inputs/15_example"));

        assert_eq!(g.width(), 10);
        assert_eq!(g.height(), 10);
        assert_eq!(g.at_xy(0, 0).unwrap(), &1);
        assert_eq!(g.at_xy(2, 0).unwrap(), &6);
    }
}
