use std::fs;

mod day_01;
mod day_02;
mod day_03;

fn main() {
    // let input_01 = fs::read_to_string("inputs/01").unwrap();
    // println!("{}", day_01::solve_part2(&input_01))
    // let input_02 = fs::read_to_string("inputs/02").unwrap();
    // println!("{}", day_02::solve_part_two(&input_02))
    let input_03 = fs::read_to_string("inputs/03.in").unwrap();
    println!("{}", day_03::solve_part_two(&input_03))
}
