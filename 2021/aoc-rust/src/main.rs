use std::time::Instant;
                                                           
             

// mod day_04;
mod day_05;

macro_rules! time_day {
    ($day_call:expr) => {
        let start = Instant::now();
        let output = $day_call;
        let duration = start.elapsed();
        println!("{}", output);
        println!("Took {:?} (real time)", duration);
    };
    ($day_call:expr, $repeats: literal) => {
        let total: Duration = (0..$repeats)
        .map(|iteration| {
            let start = Instant::now();
            let output = $day_call;
            let duration = start.elapsed();
            if iteration == 0 {
                println!("{}", output);
                print!("Repeating runs: ");
            }
            print!(".");
            io::stdout().flush().ok().expect("Could not flush stdout");
            duration
        })
        .sum();
        println!("\nTook {:?} (real time on average from {} executions)", total / $repeats, $repeats);
    };
}


fn main() {
    // time_day!(day_04::first_part(include_str!("../inputs/04")), 10);
    // time_day!(day_05::first_part(include_str!("../inputs/05")));
    time_day!(day_05::second_part(include_str!("../inputs/05")));
}
