use std::{cmp, collections::HashMap, os::linux::raw::stat, ptr::addr_eq};

use itertools::Itertools;
use rustc_hash::FxHashMap;

const WIN_THRESHOLD: i32 = 1000;
const WIN_THRESHOLD_PART_2: i32 = 21;

pub fn first_part(input: &str) -> i32 {
    let (p1, p2) = parse(input);

    let mut p1_position = p1 - 1;
    let mut p2_position = p2 - 1;
    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut is_first_player_turn = true;

    let mut die_state = 0;
    let mut n_die_throws = 0;

    while p1_score < WIN_THRESHOLD && p2_score < WIN_THRESHOLD {

        let mut addition = 0;
        for i in 0..3 {
            n_die_throws += 1;
            addition += die_state + 1;
            die_state = (die_state + 1) % 100;
        }

        if is_first_player_turn {
            p1_position = (p1_position + addition) % 10;
            p1_score += p1_position + 1;
        } else {
            p2_position = (p2_position + addition) % 10;
            p2_score += p2_position + 1;
        }

        is_first_player_turn = !is_first_player_turn;
    }

    cmp::min(p1_score, p2_score) * n_die_throws
}

pub fn second_part(input: &str) -> i64 {
    let (p1, p2) = parse(input);

    let mut memoization: FxHashMap<GameState, i64> = FxHashMap::default();
    compute_p1_wins_recursively(GameState {
        p1_pos: p1 - 1,
        p1_score: 0,
        p2_pos: p2 - 1,
        p2_score: 0,
        is_p1s_turn: true,
    }, &mut memoization)
}

fn next_state_frequencies(state: GameState) -> FxHashMap<GameState, i64> {
    let dirac_die = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    dirac_die
        .into_iter()
        .map(|(amount, freq)| {
            if state.is_p1s_turn {
                let new_pos = (state.p1_pos + amount) % 10;
                (
                    GameState {
                        p1_pos: new_pos,
                        p1_score: state.p1_score + (new_pos + 1),
                        p2_pos: state.p2_pos,
                        p2_score: state.p2_score,
                        is_p1s_turn: false,
                    },
                    freq,
                )
            } else {
                let new_pos = (state.p2_pos + amount) % 10;
                (
                    GameState {
                        p1_pos: state.p1_pos,
                        p1_score: state.p1_score,
                        p2_pos: new_pos,
                        p2_score: state.p2_score + (new_pos + 1),
                        is_p1s_turn: true,
                    },
                    freq,
                )
            }
        })
        .collect::<FxHashMap<GameState, i64>>()
}

fn compute_p1_wins_recursively(
    state: GameState,
    memoization: &mut FxHashMap<GameState, i64>,
) -> i64 {
    if let Some(mem_value) = memoization.get(&state) {
        return *mem_value;
    }

    let n_wins = next_state_frequencies(state)
        .into_iter()
        .map(|(next_state, freq)| {
            if state.is_p1s_turn {
                if next_state.is_over() {
                    freq
                } else {
                    freq * compute_p1_wins_recursively(next_state, memoization)
                }
            } else {
                if next_state.is_over() {
                    0
                } else {
                    freq * compute_p1_wins_recursively(next_state, memoization)
                }
            }
        })
        .sum::<i64>();

    memoization.entry(state).or_insert(n_wins).to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct GameState {
    p1_pos: i32,
    p1_score: i32,
    p2_pos: i32,
    p2_score: i32,
    is_p1s_turn: bool,
}

impl GameState {
    fn is_over(self: &GameState) -> bool {
        self.p1_score >= WIN_THRESHOLD_PART_2 || self.p2_score >= WIN_THRESHOLD_PART_2
    }
}

fn parse(input: &str) -> (i32, i32) {
    let (first_line, second_line) = input.split_once('\n').unwrap();
    (
        first_line
            .split_once("position: ")
            .unwrap()
            .1
            .trim()
            .parse::<i32>()
            .unwrap(),
        second_line
            .split_once("position: ")
            .unwrap()
            .1
            .trim()
            .parse::<i32>()
            .unwrap(),
    )
}

#[cfg(test)]
mod tests_day_21 {
    use std::collections::HashSet;

    use crate::day_21::{first_part, parse, second_part, GameState};

    use super::next_state_frequencies;

    const EXAMPLE_INPUT: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8\n";

    #[test]
    fn test_parse() {
        assert_eq!(parse(EXAMPLE_INPUT), (4, 8));
    }

    #[test]
    fn test_example_first_part() {
        assert_eq!(first_part(EXAMPLE_INPUT), 739785);
    }

    #[test]
    fn test_first_part() {
        assert_eq!(first_part(include_str!("../inputs/21.in")), 605070);
    }

    #[test]
    fn test_dirac() {
        let possible_states = next_state_frequencies(GameState {
            p1_pos: 3,
            p1_score: 5,
            p2_pos: 2,
            p2_score: 2,
            is_p1s_turn: true,
        });
        assert!(possible_states.keys().all(|s| !s.is_p1s_turn));
        assert!(possible_states.keys().all(|s| s.p2_pos == 2));
        assert!(possible_states.keys().all(|s| s.p2_score == 2));
        assert!(possible_states
            .keys()
            .all(|s| s.p1_score > 5 && s.p1_score <= 15));
        assert_eq!(
            possible_states.values().collect::<HashSet<&i64>>(),
            [1, 3, 6, 7].iter().collect::<HashSet<&i64>>()
        );
    }

    #[test]
    fn test_example_second_part() {
    
        assert_eq!(second_part(EXAMPLE_INPUT), 444356092776315);
    }

    #[test]
    fn test_second_part() {
    
        assert_eq!(second_part(include_str!("../inputs/21.in")), -1);
    }
}
