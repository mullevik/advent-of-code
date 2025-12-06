use crate::commons::Vec2;

pub fn p1(input: &str) -> i64 {
    let (nums, ops) = parse(input);

    let dim = Vec2::new(nums.first().unwrap().len(), nums.len());

    let mut total = 0;

    for x in 0..dim.x {
        let mut column_total = if ops[x] == "*" { 1 } else { 0 };

        let op = if ops[x] == "*" {
            |a: i64, b: i64| a * b
        } else {
            |a: i64, b: i64| a + b
        };

        for y in 0..dim.y {
            column_total = op(column_total, nums[y][x]);
        }
        total += column_total;
    }

    total
}

fn parse(input: &str) -> (Vec<Vec<i64>>, Vec<&str>) {
    let lines = input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<&str>>();

    let nums = lines[0..lines.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|p| p.trim())
        .collect::<Vec<&str>>();

    (nums, ops)
}

mod tests {
    use std::fs;

    use crate::day_06::p1;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("inputs/06.example").unwrap();
        assert_eq!(p1(&input), 4277556);
    }
}
