use core::{fmt, panic};
use std::{char, fmt::Debug, iter::Peekable, rc::Rc};

use itertools::Itertools;

pub fn first_part(input: &str) -> i64 {
    let numbers = parse(input);

    let result = numbers
        .into_iter()
        .reduce(|a, b| add(a.clone(), b.clone()))
        .unwrap();

    result.magnitude()
}

pub fn second_part(input: &str) -> i64 {
    let numbers = parse(input);

    numbers
        .iter()
        .cloned()
        .cartesian_product(numbers.iter().cloned())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| add(a, b).magnitude())
        .max().unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SnailFishNumber {
    lhs: Rc<SnailFishNode>,
    rhs: Rc<SnailFishNode>,
}
#[derive(PartialEq, Eq, Clone)]
enum SnailFishNode {
    Literal(i32),
    Nested(SnailFishNumber),
}

impl SnailFishNode {
    fn magnitude(&self) -> i64 {
        match self {
            SnailFishNode::Literal(lit) => *lit as i64,
            SnailFishNode::Nested(nested) => {
                3 * nested.lhs.magnitude() + 2 * nested.rhs.magnitude()
            }
        }
    }
}

impl fmt::Debug for SnailFishNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishNode::Literal(lit) => {
                write!(f, "{}", lit)?;
            }
            SnailFishNode::Nested(nested) => {
                write!(f, "[")?;
                nested.lhs.as_ref().fmt(f)?;
                write!(f, ",")?;
                nested.rhs.as_ref().fmt(f)?;
                write!(f, "]")?;
            }
        };
        Ok(())
    }
}

fn add(a: Rc<SnailFishNode>, b: Rc<SnailFishNode>) -> Rc<SnailFishNode> {
    reduce(Rc::new(SnailFishNode::Nested(SnailFishNumber {
        lhs: a,
        rhs: b,
    })))
}

fn reduce(n: Rc<SnailFishNode>) -> Rc<SnailFishNode> {
    let mut new_n = n.clone();
    loop {
        let (x, n_operations) = explode(new_n.clone());

        if n_operations > 0 {
            new_n = x.clone();
            continue;
        }

        let (x, n_operations) = split(new_n.clone(), 0);

        if n_operations > 0 {
            new_n = x.clone();
            continue;
        }

        break;
    }

    new_n
}

fn explode(n: Rc<SnailFishNode>) -> (Rc<SnailFishNode>, i32) {
    let mut new_n = n.clone();

    let mut to_explode = {
        match find_first_explode_node(new_n.clone(), 0) {
            Some(_n) => _n,
            None => return (n, 0),
        }
    };

    let lhs: i32 = {
        match to_explode.as_ref() {
            SnailFishNode::Literal(_) => panic!("Exploded node is not supposed to be literal"),
            SnailFishNode::Nested(nested) => match nested.lhs.as_ref() {
                SnailFishNode::Literal(lit) => *lit,
                SnailFishNode::Nested(_) => {
                    panic!("Lhs of exploded node is supposed to be literal.")
                }
            },
        }
    };
    let rhs: i32 = {
        match to_explode.as_ref() {
            SnailFishNode::Literal(_) => panic!("Exploded node is not supposed to be literal"),
            SnailFishNode::Nested(nested) => match nested.rhs.as_ref() {
                SnailFishNode::Literal(lit) => *lit,
                SnailFishNode::Nested(_) => {
                    panic!("Lhs of exploded node is supposed to be literal.")
                }
            },
        }
    };

    let maybe_prev_lit =
        find_prev_literal_before_target(new_n.clone(), to_explode.clone(), false).0;

    if let Some(prev_lit) = maybe_prev_lit {
        let new_prev = {
            match prev_lit.as_ref() {
                SnailFishNode::Literal(lit) => lit,
                SnailFishNode::Nested(_) => panic!("Found prev should be a lit"),
            }
        };
        new_n = modify(
            new_n.clone(),
            prev_lit.clone(),
            Rc::new(SnailFishNode::Literal(new_prev + lhs)),
        );
    }

    to_explode = find_first_explode_node(new_n.clone(), 0).unwrap();

    let maybe_next_lit = find_next_literal_after_target(new_n.clone(), to_explode.clone(), false).0;
    if let Some(next_lit) = maybe_next_lit {
        let new_next = {
            match next_lit.as_ref() {
                SnailFishNode::Literal(lit) => lit,
                SnailFishNode::Nested(_) => panic!("Found next should be a lit"),
            }
        };
        new_n = modify(
            new_n.clone(),
            next_lit.clone(),
            Rc::new(SnailFishNode::Literal(new_next + rhs)),
        );
    }

    to_explode = find_first_explode_node(new_n.clone(), 0).unwrap();

    new_n = modify(
        new_n.clone(),
        to_explode.clone(),
        Rc::new(SnailFishNode::Literal(0)),
    );

    (new_n, 1)
}

fn split(n: Rc<SnailFishNode>, n_splits: i32) -> (Rc<SnailFishNode>, i32) {
    match n.as_ref() {
        SnailFishNode::Literal(literal) => {
            if literal >= &10 && n_splits == 0 {
                let lhs = literal / 2;
                let rhs = if literal % 2 == 0 { lhs } else { lhs + 1 };
                (
                    Rc::new(SnailFishNode::Nested(SnailFishNumber {
                        lhs: Rc::new(SnailFishNode::Literal(lhs)),
                        rhs: Rc::new(SnailFishNode::Literal(rhs)),
                    })),
                    1,
                )
            } else {
                (n, n_splits)
            }
        }
        SnailFishNode::Nested(nested) => {
            let (lhs, n_lhs) = split(nested.lhs.clone(), n_splits);
            let (rhs, n_both) = split(nested.rhs.clone(), n_lhs);
            (
                Rc::new(SnailFishNode::Nested(SnailFishNumber { lhs, rhs })),
                n_both,
            )
        }
    }
}

fn find_first_explode_node(n: Rc<SnailFishNode>, nesting_level: i32) -> Option<Rc<SnailFishNode>> {
    match n.as_ref() {
        SnailFishNode::Literal(_) => None,
        SnailFishNode::Nested(nested) => {
            if nesting_level >= 3 && matches!(*nested.lhs, SnailFishNode::Nested(_)) {
                return Some(nested.lhs.clone());
            }
            if nesting_level >= 3 && matches!(*nested.rhs, SnailFishNode::Nested(_)) {
                return Some(nested.rhs.clone());
            }
            if let Some(lhs) = find_first_explode_node(nested.lhs.clone(), nesting_level + 1) {
                return Some(lhs);
            } else {
            }
            if let Some(rhs) = find_first_explode_node(nested.rhs.clone(), nesting_level + 1) {
                return Some(rhs);
            } else {
            }
            None
        }
    }
}

fn modify(
    n: Rc<SnailFishNode>,
    to_replace: Rc<SnailFishNode>,
    replace_with: Rc<SnailFishNode>,
) -> Rc<SnailFishNode> {
    match n.as_ref() {
        SnailFishNode::Literal(_) => n,
        SnailFishNode::Nested(nested) => {
            if Rc::as_ptr(&nested.rhs) == Rc::as_ptr(&to_replace) {
                Rc::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: nested.lhs.clone(),
                    rhs: replace_with.clone(),
                }))
            } else if Rc::as_ptr(&nested.lhs) == Rc::as_ptr(&to_replace) {
                Rc::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: replace_with.clone(),
                    rhs: nested.rhs.clone(),
                }))
            } else {
                Rc::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: modify(nested.lhs.clone(), to_replace.clone(), replace_with.clone()),
                    rhs: modify(nested.rhs.clone(), to_replace.clone(), replace_with.clone()),
                }))
            }
        }
    }
}

fn find_next_literal_after_target(
    n: Rc<SnailFishNode>,
    target: Rc<SnailFishNode>,
    target_found: bool,
) -> (Option<Rc<SnailFishNode>>, bool) {
    match n.as_ref() {
        SnailFishNode::Literal(_) => {
            if target_found {
                (Some(n.clone()), target_found)
            } else {
                (None, target_found)
            }
        }
        SnailFishNode::Nested(nested) => {
            let is_target = Rc::as_ptr(&n) == Rc::as_ptr(&target);

            if is_target {
                return (None, true);
            } else {
                let (_match, is_target_found) = find_next_literal_after_target(
                    nested.lhs.clone(),
                    target.clone(),
                    target_found,
                );
                if let Some(_m) = _match {
                    return (Some(_m), is_target_found);
                }

                return find_next_literal_after_target(
                    nested.rhs.clone(),
                    target.clone(),
                    is_target_found,
                );
            }
        }
    }
}
fn find_prev_literal_before_target(
    n: Rc<SnailFishNode>,
    target: Rc<SnailFishNode>,
    target_found: bool,
) -> (Option<Rc<SnailFishNode>>, bool) {
    match n.as_ref() {
        SnailFishNode::Literal(_) => {
            if target_found {
                (Some(n.clone()), target_found)
            } else {
                (None, target_found)
            }
        }
        SnailFishNode::Nested(nested) => {
            let is_target = Rc::as_ptr(&n) == Rc::as_ptr(&target);

            if is_target {
                return (None, true);
            } else {
                let (_match, is_target_found) = find_prev_literal_before_target(
                    nested.rhs.clone(),
                    target.clone(),
                    target_found,
                );
                if let Some(_m) = _match {
                    return (Some(_m), is_target_found);
                }

                return find_prev_literal_before_target(
                    nested.lhs.clone(),
                    target.clone(),
                    is_target_found,
                );
            }
        }
    }
}
fn parse_snailfish_numbfer(input: &str) -> Rc<SnailFishNode> {
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
                lhs: Rc::new(lhs),
                rhs: Rc::new(rhs),
            };
            stack.push(SnailFishNode::Nested(new_num));
        } else {
            input_iter.next();
        }
    }

    Rc::new(stack.pop().expect("invalid stack at EOF"))
}

fn parse_i32(input: &mut Peekable<impl Iterator<Item = char>>) -> i32 {
    let mut stack = "".to_string();

    while input.peek().is_some_and(|ch| ch.is_digit(10)) {
        stack.push(input.next().unwrap());
    }
    stack.parse::<i32>().unwrap()
}

fn parse(input: &str) -> Vec<Rc<SnailFishNode>> {
    input
        .lines()
        .filter(|line| line.contains("["))
        .map(|line| parse_snailfish_numbfer(line))
        .collect()
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
                lhs: Rc::new(SnailFishNode::Literal(1)),
                rhs: Rc::new(SnailFishNode::Literal(2)),
            }),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Rc::new(SnailFishNode::Literal(1)),
                rhs: Rc::new(SnailFishNode::Literal(2)),
            })
        );
    }

    #[test]
    fn test_parse_sf() {
        assert_eq!(
            *parse_snailfish_numbfer("[1, 2]"),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Rc::new(SnailFishNode::Literal(1)),
                rhs: Rc::new(SnailFishNode::Literal(2))
            })
        );
        assert_eq!(
            *parse_snailfish_numbfer("[9,[8,7]]"),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Rc::new(SnailFishNode::Literal(9)),
                rhs: Rc::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: Rc::new(SnailFishNode::Literal(8)),
                    rhs: Rc::new(SnailFishNode::Literal(7)),
                })),
            })
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            split(parse_snailfish_numbfer("[11,[18,7]]"), 0),
            (
                Rc::new(SnailFishNode::Nested(SnailFishNumber {
                    lhs: Rc::new(SnailFishNode::Nested(SnailFishNumber {
                        lhs: Rc::new(SnailFishNode::Literal(5)),
                        rhs: Rc::new(SnailFishNode::Literal(6)),
                    })),
                    rhs: Rc::new(SnailFishNode::Nested(SnailFishNumber {
                        lhs: Rc::new(SnailFishNode::Literal(18)),
                        rhs: Rc::new(SnailFishNode::Literal(7)),
                    })),
                })),
                1
            )
        );
    }

    #[test]
    fn test_find_first_explode() {
        assert_eq!(
            *find_first_explode_node(parse_snailfish_numbfer("[[[[[9,8],1],2],3],4]"), 0).unwrap(),
            SnailFishNode::Nested(SnailFishNumber {
                lhs: Rc::new(SnailFishNode::Literal(9)),
                rhs: Rc::new(SnailFishNode::Literal(8))
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

        if let (SnailFishNode::Nested(nested_a), SnailFishNode::Nested(nested_b)) =
            (a.as_ref(), b.as_ref())
        {
            let to_replace = nested_a.rhs.clone();
            let replace_with = nested_b.lhs.clone();
            assert_eq!(
                modify(a, to_replace, replace_with),
                parse_snailfish_numbfer("[1,[4,5]]")
            )
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_find_after_target() {
        let sn = parse_snailfish_numbfer("[[[2,[1,[9,8]]], 5], 6]");
        let en = find_first_explode_node(sn.clone(), 0).unwrap();

        assert_eq!(
            find_next_literal_after_target(sn.clone(), en.clone(), false).0,
            Some(Rc::new(SnailFishNode::Literal(5)))
        );

        assert_eq!(
            find_prev_literal_before_target(sn.clone(), en.clone(), false).0,
            Some(Rc::new(SnailFishNode::Literal(1)))
        );
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            explode(parse_snailfish_numbfer("[[[[[9,8],1],2],3],4]")).0,
            parse_snailfish_numbfer("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            explode(parse_snailfish_numbfer("[7,[6,[5,[4,[3,2]]]]]")).0,
            parse_snailfish_numbfer("[7,[6,[5,[7,0]]]]")
        );
        assert_eq!(
            explode(parse_snailfish_numbfer(
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            ))
            .0,
            parse_snailfish_numbfer("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
        );
    }

    #[test]
    fn test_addition() {
        let a = parse_snailfish_numbfer("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let b = parse_snailfish_numbfer("[1,1]");

        assert_eq!(
            add(a, b),
            parse_snailfish_numbfer("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(include_str!("../inputs/18_example")), 4140);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/18.in")), 3051);
    }

    #[test]
    fn test_second_part() {
        assert_eq!(second_part(include_str!("../inputs/18.in")), 4812);
    }

}
