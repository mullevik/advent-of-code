mod benchmark;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
// mod day_116;
mod day_17;
mod day_18;
mod day_19;
mod graph;
mod grid;
mod space;
mod utils;

use crate::benchmark::{benchmark_run, print_day, print_header};

fn main() {
    benchmark_all!(
        day_04, day_05, day_06, day_07, day_08, day_09, day_12, day_13, day_14, day_15, day_16,
        day_17, day_19
    )
}
