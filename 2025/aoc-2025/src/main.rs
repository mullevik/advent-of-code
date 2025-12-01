use std::fs;

mod day_01;

fn main() {
    // let input_01 = fs::read_to_string("inputs/01_example").unwrap();
    let input_01 = fs::read_to_string("inputs/01").unwrap();
    println!("{}", day_01::solve_part2(&input_01))
}
