use std::collections::HashMap;


fn aoc_hash(text: &str) -> i64 {
    let mut hash: i64 = 0;
    for c in text.chars() {
        let code = c as u8;
        hash += code as i64;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn first_part(input: &str) -> i64 {
    input.replace("\n", "").split(",").map(
        |text| aoc_hash(text)
    )
    .sum()
}

enum Operation {
    EQ(i32),
    DASH,
}

fn parse(text: &str) -> (String, Operation) {
    if text.contains("=") {
        let (label, amount) = text.split_once("=").unwrap();
        (label.to_string(), Operation::EQ(amount.parse::<i32>().unwrap()))
    } else {
        (text.replace("-", "").to_string(), Operation::DASH)
    }
}

pub fn second_part(input: &str) -> i32 {
    let commands: Vec<(String, Operation)> = input.replace("\n", "").split(",").map(
        |text| parse(text)
    ).collect();

    let mut boxes: Vec<Vec<(String, i32)>> = vec![vec![]; 256];

    for cmd in commands.iter() {
        let index = aoc_hash(cmd.0.as_str());
        match cmd.1 {
            Operation::EQ(focus) => {
                let one_box = boxes.get_mut(index as usize).unwrap();
                let position = one_box.iter().position(|(l, f)| l == &cmd.0);
                match position {
                    Some(p) => {
                        one_box[p] = (cmd.0.clone(), focus)
                    },
                    None => one_box.push((cmd.0.clone(), focus)),
                }
            },
            Operation::DASH => {
                let one_box = boxes.get_mut(index as usize).unwrap();
                let position = one_box.iter().position(|(l, f)| l == &cmd.0);
                match position {
                    Some(p) => {
                        one_box.remove(p);
                    },
                    None => (),
                }
            }
        }
    }

    boxes.iter().enumerate().map(
        |(box_index, one_box)| one_box.iter().enumerate().map(
            |(lens_index, lens)| ((box_index as i32 + 1) * (lens_index as i32 + 1) * (lens.1 as i32)) as i32
        ).sum::<i32>()
    ).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_15::{first_part, second_part, aoc_hash};
    
    #[test]
    fn test_some() {
        assert_eq!(aoc_hash("rn"), 0);
        assert_eq!(aoc_hash("qp"), 1);
        assert_eq!(aoc_hash("cm"), 0);
    }

    #[test]
    fn test_example() {
        assert_eq!(first_part(include_str!("inputs/15_example_1.txt")), 1320);
        assert_eq!(second_part(include_str!("inputs/15_example_1.txt")), 145);
    }
    
    // #[test]
    // fn test_parts() {
    //     assert_eq!(first_part(include_str!("inputs/15.secret")), 0);
    //     assert_eq!(second_part(include_str!("inputs/15.secret")), 0);
    // }
}