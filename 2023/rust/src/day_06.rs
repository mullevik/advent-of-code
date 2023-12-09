

fn parse_numbers(text: &str) -> Vec<u64> {
    text.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn parse_number(text: &str) -> Vec<u64> {
    vec![text.replace(" ", "").to_owned().parse::<u64>().unwrap()]
}

fn count_ways(time: u64, distance_to_beat: u64) -> i32 {
    let mut distances: Vec<u64> = vec![];
    for i in 1..time {
        distances.push((time - i) * i);
    }
    distances.into_iter().filter(|d| d > &distance_to_beat).count() as i32
}

fn compute_ways_for_races(input: &str, parsing_fn: fn(&str) -> Vec<u64>) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let times = parsing_fn(lines[0].split_once(":").unwrap().1);
    let distances_to_beat = parsing_fn(lines[1].split_once(":").unwrap().1);
    times.iter().zip(distances_to_beat.iter()).map(|it| count_ways(*it.0, *it.1)).product()
}

pub fn first_part(input: &str) -> i32 {
    compute_ways_for_races(input, parse_numbers)
}

pub fn second_part(input: &str) -> i32 {
    compute_ways_for_races(input, parse_number)
}

#[cfg(test)]
mod tests {
    use crate::day_06::{first_part, second_part, parse_numbers, count_ways, parse_number};

    #[test]
    fn test_parsing() {
        let lines: Vec<&str> = include_str!("inputs/06_example_1.txt").lines().collect();
        let times = parse_numbers(lines[0].split_once(":").unwrap().1);
        let distances_to_beat = parse_numbers(lines[1].split_once(":").unwrap().1);
        assert_eq!(times, vec![7, 15, 30]);
        assert_eq!(distances_to_beat, vec![9, 40, 200]);
        let full_time = parse_number(lines[0].split_once(":").unwrap().1);
        let full_distances = parse_number(lines[1].split_once(":").unwrap().1);
        assert_eq!(full_time[0], 71530);
        assert_eq!(full_distances[0], 940200);
    }
    #[test]
    fn test_example() {
        assert_eq!(count_ways(7, 9), 4);
        assert_eq!(first_part(include_str!("inputs/06_example_1.txt")), 288);
        assert_eq!(second_part(include_str!("inputs/06_example_1.txt")), 71503);
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/06.secret")), 32076);
        // assert_eq!(second_part(include_str!("inputs/06.secret")), 0);
    }
}