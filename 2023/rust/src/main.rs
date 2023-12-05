mod day_02;
mod day_01;

fn main() {
    // let solution = day_01::first_part();
    // let solution = day_01::second_part();
    // let solution = day_02::first_part(include_str!("inputs/02.secret"));
    let solution = day_02::second_part(include_str!("inputs/02.secret"));
    println!("{solution}");
}
