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

pub fn p2(input: &str) -> i64 {
    let lines = input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<&str>>();

    let (col_sizes, ops) = parse_ops(lines.last().unwrap());

    let mat = parse_columns(&lines[0..lines.len() - 1], &col_sizes);

    let n_rows = mat.len();
    let n_cols = mat.first().unwrap().len();

    (0..n_cols)
        .zip(ops)
        .map(|(c, op)| {
            let mut column_total = if op == '+' { 0 } else { 1 };
            let op_fn = if op == '*' {
                |a: i64, b: i64| a * b
            } else {
                |a: i64, b: i64| a + b
            };
            let col_size = col_sizes[c] as usize;
            for i in 0..col_size {
                let mut num_string = "".to_string();
                for r in 0..n_rows {
                    num_string.push(mat[r][c][i]);
                }

                let num_s = num_string.replace(" ", "");
                let num = num_s.parse::<i64>().unwrap();
                column_total = op_fn(column_total, num);
            }
            column_total
        })
        .sum()
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

fn parse_ops(op_line: &str) -> (Vec<i64>, Vec<char>) {
    let mut column_sizes = vec![];
    let mut column_ops = vec![];

    for c in op_line.chars() {
        if c == '*' {
            column_sizes.push(1);
            column_ops.push('*');
        } else if c == '+' {
            column_sizes.push(1);
            column_ops.push('+');
        } else {
            let m = column_sizes.len();
            column_sizes[m - 1] = column_sizes.last().unwrap() + 1;
        }
    }

    column_sizes.iter_mut().for_each(|x| *x = *x - 1); // subtract one for the empty space between columns
    *column_sizes.last_mut().unwrap() = *column_sizes.last_mut().unwrap() + 1; // but not for the very last column

    (column_sizes, column_ops)
}

fn parse_columns(num_lines: &[&str], col_sizes: &Vec<i64>) -> Vec<Vec<Vec<char>>> {
    num_lines
        .iter()
        .map(|line| {
            let mut row = vec![];

            let mut remaining_line: String = line.to_string();

            for c in col_sizes.iter() {
                let mut nums = vec![];

                for _ in 0..*c {
                    let char: char = remaining_line.chars().next().unwrap();
                    remaining_line.remove(0);

                    nums.push(char);
                }

                if !remaining_line.is_empty() {
                    remaining_line.remove(0);
                }
                row.push(nums);
            }
            row
        })
        .collect::<Vec<_>>()
}

mod tests {
    use std::fs;

    use crate::day_06::{p1, p2, parse_columns, parse_ops};

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("inputs/06.example").unwrap();
        assert_eq!(p1(&input), 4277556);
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("inputs/06.example").unwrap();
        assert_eq!(p2(&input), 3263827);
    }

    #[test]
    fn test_parse_columns() {
        let op_row = "+  * +   * ";
        let (col_sizes, ops) = parse_ops(op_row);

        let num_lines = vec!["11 2 333 44", " 1 2 3   4 "];
        let mat = parse_columns(&num_lines, &col_sizes);

        assert_eq!(col_sizes, vec![2, 1, 3, 2]);
        assert_eq!(ops, vec!['+', '*', '+', '*']);
        assert_eq!(
            mat,
            vec![
                vec![
                    vec!['1', '1'],
                    vec!['2'],
                    vec!['3', '3', '3'],
                    vec!['4', '4']
                ],
                vec![
                    vec![' ', '1'],
                    vec!['2'],
                    vec!['3', ' ', ' '],
                    vec!['4', ' ']
                ]
            ]
        );
    }
}
