use rand::{random_bool, random_range};

pub fn generate(n_rows: i32, n_col_nums: i32, max_col_size: i32) -> String {
    let col_sizes = (0..n_col_nums)
        .map(|_| random_range(1..=max_col_size))
        .collect::<Vec<_>>();

    let ops = col_sizes
        .iter()
        .map(|_| if random_bool(0.5) { '+' } else { '*' })
        .collect::<Vec<char>>();

    let rows = (0..n_rows)
        .map(|_| generate_row(&col_sizes))
        .collect::<Vec<_>>()
        .join("\n");

    let ops_string = ops
        .iter()
        .zip(col_sizes)
        .map(|(op, col_size)| format!("{}{}", op, " ".repeat((col_size - 1) as usize)))
        .collect::<Vec<String>>()
        .join(" ");

    format!("{}\n{}", rows, ops_string)
}

fn generate_row(col_sizes: &[i32]) -> String {
    col_sizes
        .iter()
        .map(|&c| {
            let num_size = random_range(1..=c);
            let rest = c - num_size;
            let offset = random_range(0..=rest);
            let num = random_range(1..(10 as i64).pow(num_size as u32));
            let num_string = format!("{}", num);
            format!(
                "{}{}{}",
                " ".repeat(offset as usize),
                num_string,
                " ".repeat((c - offset - num_string.len() as i32) as usize)
            )
        })
        .collect::<Vec<String>>()
        .join(" ")
}

mod tests {
    use crate::gen_06::generate;

    #[test]
    fn test_generate() {
        let input = generate(3, 3, 5);
        assert_eq!(input.split("\n").count(), 4);
    }
}
