use std::collections::HashMap;

pub fn first_part(input: &str) -> i32 {
    unimplemented!()
}

pub fn second_part(input: &str) -> i32 {
    unimplemented!()
}

fn parse(input: &str) -> (String, HashMap<String, String>) {
    let (polymer, rules) = input.split_once("\n\n").unwrap();

    (polymer.to_string(), parse_rules(rules))
}

fn parse_rules(input: &str) -> HashMap<String, String> {
    input
        .lines()
        .map(|row| {
            let (from, to) = row.split_once("->").unwrap();
            (from.trim().to_string(), to.trim().to_string())
        })
        .collect::<HashMap<String, String>>()
}

mod tests_day_14 {
    use crate::day_14::parse;

    #[test]
    fn test_parse() {
        let (polymer, rules) = parse(include_str!("../inputs/14_example"));

        assert_eq!(polymer, "NNCB".to_string());
        assert_eq!(rules.len(), 16);
        assert_eq!(rules.get(&"CN".to_string()), Some("C".to_string()).as_ref());
    }
    // add code here
}
