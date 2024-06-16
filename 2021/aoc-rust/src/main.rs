mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod utils;
mod benchmark;

use crate::benchmark::{benchmark_run, print_day, print_header};

fn main() {
    benchmark_all!(day_04, day_05, day_06, day_07, day_08)
}
