

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Move {
    ROCK,
    PAPER,
    SCISSORS
}
impl Move {
    fn score(&self) -> i32 {
        match self {
            Move::ROCK => 1,
            Move::PAPER => 2,
            Move::SCISSORS => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Outcome {
    WIN,
    LOSS,
    DRAW
}
impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::WIN => 6,
            Outcome::LOSS => 0,
            Outcome::DRAW => 3,
        }
    }
}

fn play(first: Move, second: Move) -> Outcome {
    if first == second {
        return Outcome::DRAW
    }
    match (first, second) {
        (Move::ROCK, Move::PAPER) => Outcome::WIN,
        (Move::ROCK, Move::SCISSORS) => Outcome::LOSS,
        (Move::PAPER, Move::ROCK) => Outcome::LOSS,
        (Move::PAPER, Move::SCISSORS) => Outcome::WIN,
        (Move::SCISSORS, Move::ROCK) => Outcome::WIN,
        (Move::SCISSORS, Move::PAPER) => Outcome::LOSS,
        _ => panic!("Invalid game {:?} vs. {:?}", first, second)
    }
}

fn letter_to_move(s: &str) -> Move {
    match s {
        "A" | "X" => Move::ROCK,
        "B" | "Y" => Move::PAPER,
        "C" | "Z" => Move::SCISSORS,
        _ => panic!("Invalid game move {}", s)
    }
}

fn letter_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::LOSS,
        "Y" => Outcome::DRAW,
        "Z" => Outcome::WIN,
        _ => panic!("Invalid game outcome {}", s)
    } 
}

fn line_to_moves(line: &str) -> (Move, Move) {
    return (letter_to_move(&line[0..1]), letter_to_move(&line[2..3]));
}

fn line_to_move_outcome(line: &str) -> (Move, Outcome) {
    return (letter_to_move(&line[0..1]), letter_to_outcome(&line[2..3]))
}

fn play_until_outcome(first: Move, wanted_outcome: Outcome) -> Move {
    for m in [Move::ROCK, Move::PAPER, Move::SCISSORS] {
        if play(first, m) == wanted_outcome {
            return m;
        }
    }
    panic!("Impossible game with opponent's {:?}", first)
}


pub fn solve() {
    let move_outcome_vec = include_str!("../inputs/02_1")
    .lines()
    .map(line_to_move_outcome)
    .collect::<Vec<_>>();
    // println!("{:?}", move_outcome_vec);
    
    let scores = move_outcome_vec
    .iter()
    .map(|m| 
        play_until_outcome(m.0, m.1).score() + m.1.score())
    .collect::<Vec<_>>();
    // println!("{scores:?}");

    println!("{:?}", scores.iter().sum::<i32>())
}