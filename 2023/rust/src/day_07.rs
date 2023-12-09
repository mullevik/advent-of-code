use std::cmp::{Ordering, Reverse};
use itertools::{Itertools, repeat_n};


use counter::Counter;

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Hash, Clone, Copy)]
enum C {
    A,
    K,
    Q,
    J,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    N1,  // joker
}


#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Clone, Copy)]
enum WinCondition {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl WinCondition {
    fn from_hand(hand: &[C;5]) -> Self {
        let counter = hand.iter().collect::<Counter<_>>();

        if counter.values().any(|x| x == &(5 as usize)) {
            return Self::FiveOfAKind
        } else if counter.values().any(|x| x == &(4 as usize)) {
            return Self::FourOfAKind
        } else if counter.values().any(|x| x == &(3 as usize)) && counter.values().any(|x| x == &(2 as usize)) {
            return Self::FullHouse
        } else if counter.values().any(|x| x == &(3 as usize)) {
            return Self::ThreeOfAKind
        } else if counter.values().filter(|x| x == &&(2 as usize)).count() == 2 {
            return Self::TwoPair
        } else if counter.values().any(|x| x == &(2 as usize)) {
            return Self::OnePair
        }
        Self::HighCard
    }
}


fn parse_cards(text: &str) -> [C;5] {
    let cards = text.chars().map(|c| match c {
        'A' => C::A,
        'K' => C::K,
        'Q' => C::Q,
        'T' => C::T,
        'J' => C::J,
        '9' => C::N9,
        '8' => C::N8,
        '7' => C::N7,
        '6' => C::N6,
        '5' => C::N5,
        '4' => C::N4,
        '3' => C::N3,
        '2' => C::N2,
        _ => panic!("Unknown character encountered at parsing")
    })
    .collect::<Vec<_>>();
    [cards[0], cards[1], cards[2], cards[3], cards[4]]
}

fn swap_joker(hand: [C;5], swap_with: C) -> [C;5] {
    let mut new_cards = hand.clone();
    for (i, card) in hand.iter().enumerate() {
        match card {
            C::J => new_cards[i] = swap_with,
            _ => {}
        }
    }
    new_cards
}

fn parse_input(text: &str) -> ([C;5], i64) {
    match text.split_once(" ") {
        Some(parts) => (parse_cards(parts.0), parts.1.parse::<i64>().unwrap()),
        None => panic!("Unable to parse input")
    }
}

fn parse_input_and_swap_joker(text: &str) -> ([C;5], i64) {
    match text.split_once(" ") {
        Some(parts) => (swap_joker(parse_cards(parts.0), C::N1), parts.1.parse::<i64>().unwrap()),
        None => panic!("Unable to parse input")
    }
}

fn compare_by_strength(lhs: &[C;5], rhs: &[C;5]) -> Ordering {    
    match 
    Reverse(WinCondition::from_hand(&lhs))
    .cmp(&Reverse(WinCondition::from_hand(&rhs))) 
    {
        Ordering::Equal => 
            Reverse(lhs)
            .cmp(&Reverse(&rhs)),
        x => x
    }
}

fn compare_by_all_possible_strengths(lhs: &[C;5], rhs: &[C;5]) -> Ordering {
    let best_left = best_hand(lhs);
    let best_right = best_hand(rhs);
    match 
    Reverse(WinCondition::from_hand(&best_left))
    .cmp(&Reverse(WinCondition::from_hand(&best_right))) 
    {
        Ordering::Equal => 
            Reverse(lhs)
            .cmp(&Reverse(&rhs)),
        x => x
    }
}

const UNIVERSE: [C;13] = [C::A, C::K, C::T, C::Q, C::J, C::N2, C::N3, C::N4, C::N5, C::N6, C::N7, C::N8, C::N9];

fn best_hand(hand: &[C;5]) -> [C;5] {
    let joker_indices: Vec<usize> = hand
    .iter()
    .enumerate()
    .filter(|(i, c)| matches!(c, C::N1))
    .map(|(i, c)| i)
    .collect();

    if joker_indices.is_empty() {
        return *hand
    }

    let mut best_win_condition = WinCondition::from_hand(&hand);
    let mut best_hand: [C;5] = hand.clone();

    for permutation in repeat_n(UNIVERSE, joker_indices.len()).multi_cartesian_product() {
        let mut new_hand = hand.clone();
        for (i, c) in permutation.iter().enumerate() {
            new_hand[joker_indices[i]] = *c;
        }
        let new_win_condition = WinCondition::from_hand(&new_hand);
        match Reverse(new_win_condition).cmp(&Reverse(best_win_condition)) {
            Ordering::Greater => {
                best_win_condition = new_win_condition;
                best_hand = new_hand.clone();
            },
            _ => ()
        } 
    }
    best_hand.clone()
}


fn parse_game_turns(input: &str, parsing_fn: fn(&str) -> ([C;5], i64)) -> Vec<([C;5], i64)> {
    input
    .split("\n")
    .filter(|l| !l.trim().is_empty())
    .map(|l| parsing_fn(l))
    .collect()
}

pub fn first_part(input: &str) -> i64 {
    let mut game_turns = parse_game_turns(input, parse_input);
    
    game_turns.sort_by(|lhs, rhs| compare_by_all_possible_strengths(&lhs.0, &rhs.0));

    game_turns
    .iter()
    .enumerate()
    .map(|(i, (_, bid))| (*bid) * (i + 1) as i64)
    .sum()
}

pub fn second_part(input: &str) -> i64 {
    let mut game_turns = parse_game_turns(input, parse_input_and_swap_joker);
    
    game_turns.sort_by(
        |lhs, rhs|
        compare_by_all_possible_strengths(&lhs.0, &rhs.0)
    );

    game_turns
    .iter()
    .enumerate()
    .map(|(i, (_, bid))| (*bid) * (i + 1) as i64)
    .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_07::*;
    
    #[test]
    fn test_ordering() {
        let mut sorted_conditions = vec![WinCondition::FullHouse, WinCondition::FiveOfAKind, WinCondition::HighCard];
        sorted_conditions.sort();
        assert_eq!(sorted_conditions, vec![WinCondition::FiveOfAKind, WinCondition::FullHouse, WinCondition::HighCard]);
        sorted_conditions.sort_by_key(|&cond| std::cmp::Reverse(cond));
        assert_eq!(sorted_conditions, vec![WinCondition::HighCard, WinCondition::FullHouse, WinCondition::FiveOfAKind]);

        let mut cards = [
            [C::N2, C::N3, C::N4, C::N5, C::N6],
            [C::N2, C::N3, C::N2, C::N5, C::N6],
            [C::N2, C::N3, C::N4, C::N5, C::A],
        ];
        cards.sort_by(compare_by_strength);

        assert_eq!(cards, [
            [C::N2, C::N3, C::N4, C::N5, C::N6],
            [C::N2, C::N3, C::N4, C::N5, C::A],
            [C::N2, C::N3, C::N2, C::N5, C::N6],
        ]);
    }

    #[test]
    fn test_win_condition() {
        assert_eq!(
            WinCondition::from_hand(&[C::N2, C::N3, C::N2, C::N5, C::N6]),
            WinCondition::OnePair,
        );
        assert_eq!(
            WinCondition::from_hand(&[C::N2, C::N3, C::N4, C::N5, C::N6]),
            WinCondition::HighCard
        );
        assert_eq!(
            WinCondition::from_hand(&[C::N2, C::N3, C::N2, C::N3, C::N3]),
            WinCondition::FullHouse
        )
    }

    #[test]
    fn test_example() {
        
        let mut game_turns = parse_game_turns(include_str!("inputs/07_example_1.txt"), parse_input);
        game_turns.sort_by(|lhs, rhs| compare_by_strength(&lhs.0, &rhs.0));
        assert_eq!(
            game_turns,
            vec![
                ([C::N3, C::N2, C::T, C::N3, C::K], 765),
                ([C::K, C::T, C::J, C::J, C::T], 220),
                ([C::K, C::K, C::N6, C::N7, C::N7], 28),
                ([C::T, C::N5, C::N5, C::J, C::N5], 684),
                ([C::Q, C::Q, C::Q, C::J, C::A], 483),
            ]
        );
        let mut joker_game_turns = parse_game_turns(include_str!("inputs/07_example_1.txt"), parse_input_and_swap_joker);
        println!("{joker_game_turns:?}");
        joker_game_turns.sort_by(|lhs: &([C; 5], i64), rhs| compare_by_all_possible_strengths(&lhs.0, &rhs.0));
        assert_eq!(
            joker_game_turns,
            vec![
                ([C::N3, C::N2, C::T, C::N3, C::K], 765),
                ([C::K, C::K, C::N6, C::N7, C::N7], 28),
                ([C::T, C::N5, C::N5, C::N1, C::N5], 684),
                ([C::Q, C::Q, C::Q, C::N1, C::A], 483),
                ([C::K, C::T, C::N1, C::N1, C::T], 220),
            ]
        );
        assert_eq!(first_part(include_str!("inputs/07_example_1.txt")), 6440);
        assert_eq!(second_part(include_str!("inputs/07_example_1.txt")), 5905);
    }

    #[test]
    fn test_best_hand() {
        assert_eq!(
            best_hand(&[C::N3, C::N2, C::N1, C::N4, C::K]),
            [C::N3, C::N2, C::K, C::N4, C::K],
        );
        assert_eq!(
            best_hand(&[C::K, C::T, C::N1, C::N1, C::T]),
            [C::K, C::T, C::T, C::T, C::T]
        );
    }
    
    #[test]
    fn test_parts() {
        assert_eq!(first_part(include_str!("inputs/07.secret")), 253313241);
        // assert_eq!(second_part(include_str!("inputs/07.secret")), 253362743);  // takes too long to run each time
    }
}