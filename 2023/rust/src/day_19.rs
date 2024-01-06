use std::{unimplemented, collections::{HashMap, HashSet}};

use itertools::{Itertools, cloned};

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

#[derive(Clone, Copy, Debug)]
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

    fn cmp(&self, lhs: i64, rhs: i64) -> bool {
        match self {
            Operator::GT => lhs > rhs,
            Operator::LT => lhs < rhs
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    fn apply(&self, part: &Part) -> String {
        let matched_deciders = self.deciders
        .iter()
        .filter(|d| match d.subject {
            PartType::X => d.operator.cmp(part.values[0], d.value),
            PartType::M => d.operator.cmp(part.values[1], d.value),
            PartType::A => d.operator.cmp(part.values[2], d.value),
            PartType::S => d.operator.cmp(part.values[3], d.value),
        })
        .take(1)
        .collect::<Vec<&Decider>>();

        if matched_deciders.is_empty() {
            print!("(fall)");
            return self.fallback_destination.clone()
        }
        let _first = matched_deciders
        .first()
        .unwrap();
        let _first_subject = _first.subject;
        print!("({_first_subject:?})");
        _first.destination
        .clone()
    }
}


fn parse_input(text: &str) -> (Vec<Rule>, Vec<Part>) {
    let (rules, parts) = text.split_once("\n\n").unwrap();
    let _rules: Vec<Rule> = rules.split("\n").map(|p| Rule::from_str(p)).collect();
    let _parts: Vec<Part> = parts.split("\n").map(|p| Part::from_str(p)).collect();
    (_rules, _parts)
}

fn is_accepted(part: &Part, rules: &HashMap<String, &Rule>) -> bool {
    let in_rule = rules.get("in").unwrap();
    println!("Accepting {part:?}");

    let mut destination = in_rule.apply(part);
    
    let terminal_destinations = ["A", "R"];

    while ! terminal_destinations.contains(&destination.as_str()) {
        print!("{destination:?} -> ");
        destination = rules.get(&destination).unwrap().apply(part);
    }
    println!("{destination:?}");
    match destination.as_str() {
        "A" => true,
        "R" => false,
        _ => panic!("Invalid terminal destination {destination:?}")
    }
}


pub fn first_part(input: &str) -> i64 {
    let (rules, parts) = parse_input(input);
    let rules_map: HashMap<String, &Rule> = rules
    .iter()
    .map(|r| (r.name.clone(), r))
    .collect();

    println!("{rules_map:?}");

    let accepted_parts: Vec<&Part> = parts
    .iter()
    .filter(|p| is_accepted(p, &rules_map))
    .collect();

    accepted_parts.iter()
    .map(|p| p.sum())
    .sum()
}

type ProductOptions = [HashSet<u16>; 4];

fn bound_options(decider: &Decider, option_index: i32, options: ProductOptions) -> (ProductOptions, ProductOptions) {
    let option = options.get(option_index as usize).unwrap();
    let filtered_option: HashSet<u16> = option.iter().filter(|v| decider.operator.cmp(**v as i64, decider.value)).cloned().collect();
    let inverse_option: HashSet<u16> = option.difference(&filtered_option).cloned().collect();
    if option_index == 0 {
        return ([filtered_option, options[1].clone(), options[2].clone(), options[3].clone()], [inverse_option, options[1].clone(), options[2].clone(), options[3].clone()]);
    } else if option_index == 1 {
        return ([options[0].clone(), filtered_option, options[2].clone(), options[3].clone()], [options[0].clone(), inverse_option, options[2].clone(), options[3].clone()]);  
    } else if option_index == 2 {
        return ([options[0].clone(), options[1].clone(), filtered_option, options[3].clone()], [options[0].clone(), options[1].clone(), inverse_option, options[3].clone()]);  
    } else if option_index == 3 {
        return ([options[0].clone(), options[1].clone(), options[2].clone(), filtered_option], [options[0].clone(), options[1].clone(), options[2].clone(), inverse_option]);  
    } else {
        panic!("Something unexpected");
    }
}

fn collect_combinations(rules_map: &HashMap<String, &Rule>, current: String, options: ProductOptions) -> i64 {

    if current == "A" {
        let prod = options.iter().map(|x| x.len() as i64).product();
        return prod;
    }

    if current == "R" {
        return 0;
    }

    let rule = rules_map.get(&current).unwrap();

    let mut fallback_options = options;
    let mut accumulator = 0;
    for d in rule.deciders.iter() {
        let (bounded_options_happy, bounded_options_unhappy) = match d.subject {
            PartType::X => {
                bound_options(d, 0, fallback_options)
            },
            PartType::M => {
                bound_options(d, 1, fallback_options)
            },
            PartType::A => {
                bound_options(d, 2, fallback_options)
            },
            PartType::S => {
                bound_options(d, 3, fallback_options)
            },
        };
        fallback_options = bounded_options_unhappy;
        accumulator += collect_combinations(rules_map, d.destination.clone(), bounded_options_happy);
    }

    accumulator += collect_combinations(rules_map, rule.fallback_destination.clone(), fallback_options);
    accumulator
}


pub fn second_part(input: &str) -> i64 {
    let (rules, parts) = parse_input(input);
    let rules_map: HashMap<String, &Rule> = rules
    .iter()
    .map(|r| (r.name.clone(), r))
    .collect();

    let options: [HashSet<u16>;4] = [(1..4001).collect(), (1..4001).collect(), (1..4001).collect(), (1..4001).collect()];

    collect_combinations(&rules_map, "in".to_string(), options)
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
        assert_eq!(first_part(include_str!("inputs/19_example_1.txt")), 19114);
        assert_eq!(second_part(include_str!("inputs/19_example_1.txt")), 167409079868000i64);
    }
    
    #[test]
    fn test_parts() {
        // unimplemented!();
        assert_eq!(first_part(include_str!("inputs/19.secret")), 472630);
        assert_eq!(second_part(include_str!("inputs/19.secret")), 116738260946855);
    }
}