use std::{char, iter::Peekable};

pub fn first_part(input: &str) -> i32 {
    todo!()
}

pub fn second_part(input: &str) -> i32 {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnailFishNumber {
    lhs: Box<SnailFishNode>,
    rhs: Box<SnailFishNode>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
enum SnailFishNode {
    Literal(i32),
    Nested(SnailFishNumber),
}

fn find_first_explode_node(n: SnailFishNode, nesting_level: i32) -> Option<Box<SnailFishNode>> {
    match n {
        SnailFishNode::Literal(_) => None,
        SnailFishNode::Nested(nested) => {
            if nesting_level >= 3 && matches!(*nested.lhs, SnailFishNode::Nested(_)) {
                return Some(nested.lhs);
            }
            if nesting_level >= 3 && matches!(*nested.rhs, SnailFishNode::Nested(_)) {
                return Some(nested.rhs);
            }
            if let Some(lhs) = find_first_explode_node(*nested.lhs, nesting_level + 1) {
                return Some(lhs);
            } else {
            }
            if let Some(rhs) = find_first_explode_node(*nested.rhs, nesting_level + 1) {
                return Some(rhs);
            } else {
            }
            None
        }
    }
}

fn modify(
    n: SnailFishNode,
    to_replace: &Box<SnailFishNode>,
    replace_with: &Box<SnailFishNode>,
) -> SnailFishNode {
    match n {
        SnailFishNode::Literal(_) => n,
        SnailFishNode::Nested(nested) => {
            if nested.lhs == *to_replace {
                SnailFishNode::Nested(SnailFishNumber {
                    lhs: (*replace_with).clone(),
                    rhs: nested.rhs,
                })
            } else if nested.rhs == *to_replace {
                SnailFishNode::Nested(SnailFishNumber {
                    lhs: nested.lhs,
                    rhs: (*replace_with).clone(),
                })
            } else {
                SnailFishNode::Nested(SnailFishNumber {
                    lhs: Box::new(modify(*nested.lhs, to_replace, replace_with)),
                    rhs: Box::new(modify(*nested.rhs, to_replace, replace_with)),
                })
            }
        }
    }
}

// fn explode(
//     n: SnailFishNode,
//     to_explode: SnailFishNumber,
//     previous: Option<SnailFishNumber>
//     previous_target_lhs: bool,
//     next: Option<i32>,
// ) -> (SnailFishNode, i32) {
//     todo!()
// }

fn split(n: SnailFishNode, n_splits: i32) -> (SnailFishNode, i32) {
    match n {
        SnailFishNode::Literal(literal) => {
            if literal >= 10 && n_splits == 0 {
                let lhs = literal / 2;
                let rhs = if literal % 2 == 0 { lhs } else { lhs + 1 };
                (
                    SnailFishNode::Nested(SnailFishNumber {
                        lhs: Box::new(SnailFishNode::Literal(lhs)),
                        rhs: Box::new(SnailFishNode::Literal(rhs)),
                    }),
                    1,
                )
            } else {
                (n, n_splits)
            }
        }
        SnailFishNode::Nested(nested) => {
            let (lhs, n_lhs) = split(*nested.lhs, n_splits);
            let (rhs, n_both) = split(*nested.rhs, n_lhs);
            (
                SnailFishNode::Nested(SnailFishNumber {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }),
                n_both,
            )
        }
    }
}

fn parse_snailfish_numbfer(input: &str) -> SnailFishNode {
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

    stack.pop().expect("invalid stack at EOF")
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
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(1)),
                rhs: Box::new(SnailFishNode::Literal(2))
            })
        );
        assert_eq!(
            parse_snailfish_numbfer("[9,[8,7]]"),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(9)),
                rhs: Box::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: Box::new(SnailFishNode::Literal(8)),
                    rhs: Box::new(SnailFishNode::Literal(7)),
                })),
            })
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            split(parse_snailfish_numbfer("[11,[18,7]]"), 0),
            (
                SnailFishNode::Nested(SnailFishNumber {
                    lhs: Box::new(SnailFishNode::Nested(SnailFishNumber {
                        lhs: Box::new(SnailFishNode::Literal(5)),
                        rhs: Box::new(SnailFishNode::Literal(6)),
                    })),
                    rhs: Box::new(SnailFishNode::Nested(SnailFishNumber {
                        lhs: Box::new(SnailFishNode::Literal(18)),
                        rhs: Box::new(SnailFishNode::Literal(7)),
                    })),
                }),
                1
            )
        );
    }

    #[test]
    fn test_find_first_explode() {
        assert_eq!(
            *find_first_explode_node(parse_snailfish_numbfer("[[[[[9,8],1],2],3],4]"), 0).unwrap(),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Box::new(SnailFishNode::Literal(9)),
                rhs: Box::new(SnailFishNode::Literal(8))
            })
        );
        assert_eq!(
            find_first_explode_node(parse_snailfish_numbfer("[[[[3,1],2],3],4]"), 0),
            None
        )
    }
    #[test]
    fn test_modify() {
        let a = parse_snailfish_numbfer("[1, [2, 3]]");
        let b = parse_snailfish_numbfer("[[4, 5], 6]");



        if let (SnailFishNode::Nested(nested_a), SnailFishNode::Nested(nested_b)) = (a.clone(), b) {
            let to_replace = nested_a.rhs;
            let replace_with = nested_b.lhs;
            assert_eq!(
                modify(a, &to_replace, &replace_with),
                parse_snailfish_numbfer("[1,[4,5]]")
            )
        } else {
            assert!(false);
        }
    }
}
