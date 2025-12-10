use std::fs;

mod commons;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_09;
mod gen_06;
fn main() {
    // let input_01 = fs::read_to_string("inputs/01").unwrap();
    // println!("{}", day_01::solve_part2(&input_01))
    // let input_02 = fs::read_to_string("inputs/02").unwrap();
    // println!("{}", day_02::solve_part_two(&input_02))
    // let input_03 = fs::read_to_string("inputs/03.in").unwrap();
    // println!("{}", day_03::solve_part_two(&input_03))
    // let input = fs::read_to_string("inputs/04.in").unwrap();
    // println!("{}", day_04::p2(&input))
    // let input = fs::read_to_string("inputs/05.in").unwrap();
    // println!("{}", day_05::p2(&input))
    // let input = fs::read_to_string("inputs/06.in").unwrap();
    // println!("{}", day_06::p2(&input))
    // println!("{}", gen_06::generate(4, 20, 4))
    // let input = fs::read_to_string("inputs/07.in").unwrap();
    // println!("{}", day_07::p2(&input))
    println!(
        "{}",
        day_09::p2(&fs::read_to_string("inputs/09.in").unwrap())
    );
}
