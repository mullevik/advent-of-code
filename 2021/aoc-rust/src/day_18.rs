use core::panic;
use std::{char, iter::Peekable};

pub fn first_part(input: &str) -> i32 {
    todo!()
}

pub fn second_part(input: &str) -> i32 {
    todo!()
}

#[derive(Debug, PartialEq, Eq)]
struct SnailFishNumber {
    lhs: Box<SnailFishNode>,
    rhs: Box<SnailFishNode>,
}
#[derive(Debug, PartialEq, Eq)]
enum SnailFishNode {
    Literal(i32),
    Nested(SnailFishNumber),
}

fn parse_snailfish_numbfer(input: &str) -> SnailFishNumber {
    let mut stack: Vec<SnailFishNode> = vec![];

    let mut input_iter = input.chars().peekable();

    while let Some(ch) = input_iter.peek() {
        if ch.is_digit(10) {
            stack.push(SnailFishNode::Literal(parse_i32(&mut input_iter)));
        } else if *ch == ']' {
            input_iter.next();
            let rhs = stack.pop().expect("rhs missing on stack");
            let lhs = stack.pop().expect("lhs missing on stack");
            let new_num = SnailFishNumber {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            };
            stack.push(SnailFishNode::Nested(new_num));
        } else {
            input_iter.next();
        }
    }

    if let Some(SnailFishNode::Nested(x)) = stack.pop() {
        x
    } else {
        panic!("invalid stack");
    }
}

fn parse_i32(input: &mut Peekable<impl Iterator<Item = char>>) -> i32 {
    let mut stack = "".to_string();

    while input.peek().is_some_and(|ch| ch.is_digit(10)) {
        stack.push(input.next().unwrap());
    }
    stack.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests_day_18 {
    use super::*;

    #[test]
    fn test_parsing() {
        let x: serde_json::Value = serde_json::from_str("[[1,2],3]").unwrap();
        assert_eq!(
            x.as_array()
                .unwrap()
                .get(0)
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|e| e.as_i64().unwrap())
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(x.as_array().unwrap().get(1).unwrap().as_i64().unwrap(), 3);
    }

    #[test]
    fn test_snailfish() {
        assert_eq!(SnailFishNode::Literal(3), SnailFishNode::Literal(3));
        assert_eq!(
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(1)),
                rhs: Box::new(SnailFishNode::Literal(2)),
            }),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(1)),
                rhs: Box::new(SnailFishNode::Literal(2)),
            })
        );
    }

    #[test]
    fn test_parse_sf() {
        assert_eq!(
            parse_snailfish_numbfer("[1, 2]"),
            SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(1)),
                rhs: Box::new(SnailFishNode::Literal(2))
            }
        );
        assert_eq!(
            parse_snailfish_numbfer("[9,[8,7]]"),
            SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(9)),
                rhs: Box::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: Box::new(SnailFishNode::Literal(8)),
                    rhs: Box::new(SnailFishNode::Literal(7)),
                })),
            }
        );
    }

}
