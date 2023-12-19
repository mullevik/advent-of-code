use std::unimplemented;

use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum PartType {
    X,
    M,
    A,
    S,
}

impl PartType {
    fn from_str(text: &str) -> Self {
        match text.trim() {
            "x" => PartType::X,
            "m" => PartType::M,
            "a" => PartType::A,
            "s" => PartType::S,
            _ => panic!("Unknown part {text:?}")
        }
    }
}

enum Operator {
    LT,
    GT,
}

impl Operator {
    fn from_str(text: &str) -> Self {
        match text.trim() {
            ">" => Operator::GT,
            "<" => Operator::LT,
            _ => panic!("Unknown operator {text:?}")
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Part {
    values: [i64; 4]
}

impl Part {
    fn sum(&self) -> i64 {
        self.values.iter().sum()
    }

    fn from_str(text: &str) -> Self {
        println!("Parsing Part from {text:?}");
        let values: Vec<i64> = text
        .strip_prefix("{").unwrap()
        .strip_suffix("}").unwrap()
        .split(",")
        .map(|slc| slc
            .split_once("=")
            .unwrap()
            .1
            .parse::<i64>()
            .unwrap()
        )
        .collect();
        Part { values: [values[0], values[1], values[2], values[3]]}
    }
}

struct Decider {
    subject: PartType,
    operator: Operator,
    value: i64,
    destination: String,
}

impl Decider {
    fn from_str(text: &str) -> Self {
        let (decider_text, destination) = text.split_once(":").unwrap();

        if text.contains("<") {
            let (subject, value) = decider_text.split_once("<").unwrap();
            return Decider {
                subject: PartType::from_str(subject),
                operator: Operator::LT,
                value: value.parse::<i64>().unwrap(),
                destination: destination.to_string(),
            }
        } else if text.contains(">") {
            let (subject, value) = decider_text.split_once(">").unwrap();
            return Decider {
                subject: PartType::from_str(subject),
                operator: Operator::GT,
                value: value.parse::<i64>().unwrap(),
                destination: destination.to_string(),
            }
        } else {
            panic!("Unknown operator in {text:?}")
        }
    }
}

struct Rule {
    name: String,
    deciders: Vec<Decider>,
    fallback_destination: String,
}

impl Rule {
    fn from_str(text: &str) -> Self {
        let (name, deciders_text) = text.split_once("{").unwrap();

        let decider_parts: Vec<&str> = deciders_text
        .strip_suffix("}").unwrap()
        .split(",")
        .collect();
        
        let (fallback, just_decider_parts, ) = decider_parts.split_last().unwrap();

        let deciders: Vec<Decider> = just_decider_parts.iter().map(|p| Decider::from_str(p)).collect();

        Self {
            name: name.to_string(),
            deciders: deciders,
            fallback_destination: fallback.to_string()
        }
    }
}


fn parse_input(text: &str) -> (Vec<Rule>, Vec<Part>) {
    let (rules, parts) = text.split_once("\n\n").unwrap();
    let _rules: Vec<Rule> = rules.split("\n").map(|p| Rule::from_str(p)).collect();
    let _parts: Vec<Part> = parts.split("\n").map(|p| Part::from_str(p)).collect();
    (_rules, _parts)
}

fn is_accepted(part: Part, rules: &Vec<Rule>) -> bool {
    unimplemented!()
}


pub fn first_part(input: &str) -> i32 {
    let (rules, parts) = parse_input(input);

    unimplemented!();
    // let accepted_parts: Vec<Part> = parts.iter()
    // .filter(|p| is_accepted(p, &rules))
    // .collect();

    // accepted_parts.iter()
    // .map(|p| p.sum())
    // .sum()
}

pub fn second_part(input: &str) -> i32 {
    return -1;
}

#[cfg(test)]
mod tests {
    use crate::day_19::{first_part, second_part, parse_input};

    #[test]
    fn test_parsing() {
        let (rules, parts) = parse_input(include_str!("inputs/19_example_1.txt"));

        assert_eq!(rules.len(), 11);
        assert_eq!(parts.len(), 5);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/19_example_1.txt")), 0);
    }
    
    #[test]
    fn test_parts() {
        unimplemented!();
        // assert_eq!(first_part(include_str!("inputs/19.secret")), 0);
        // assert_eq!(second_part(include_str!("inputs/19.secret")), 0);
    }
}