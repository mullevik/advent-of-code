use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefMutIterator, ParallelBridge, ParallelIterator,
};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = BOARD_WIDTH;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub fn first_part(input: &str) -> i32 {
    let (numbers, boards) = parse_input(input);
    let mut hit_masks: Vec<Vec<bool>> = vec![vec![false; BOARD_SIZE]; boards.len()];

    for n in numbers.iter() {
        apply_number_to_all(*n, &boards, &mut hit_masks);

        let won_scores = boards
            .iter()
            .zip(hit_masks.iter())
            .enumerate()
            .par_bridge()
            .filter(|(_, (_, mask))| is_board_winning(mask))
            .map(|(_, (board, mask))| evaluate_score(board, mask))
            .collect::<Vec<i32>>();

        if !won_scores.is_empty() {
            return won_scores.first().unwrap() * n;
        }
    }

    -1
}

pub fn second_part(input: &str) -> i32 {
    let (numbers, boards) = parse_input(input);
    let mut hit_masks: Vec<Vec<bool>> = vec![vec![false; BOARD_SIZE]; boards.len()];
    let mut win_mask: Vec<bool> = vec![false; boards.len()];

    for n in numbers.iter() {
        apply_number_to_all(*n, &boards, &mut hit_masks);

        let newly_winning_board_indices = boards
            .iter()
            .zip(hit_masks.iter().zip(win_mask.iter_mut()))
            .enumerate()
            .par_bridge()
            .filter(|(_, (_, (mask, is_won)))| (!**is_won) && is_board_winning(mask))
            .map(|(i, (_, (_, is_won)))| {
                *is_won = true;
                i
            })
            .collect::<Vec<_>>();

        if (!newly_winning_board_indices.is_empty()) && win_mask.iter().all(|x| *x) {
            let last_idx = *newly_winning_board_indices.last().unwrap();
            let last_board = &boards[last_idx];
            let last_mask = &hit_masks[last_idx];
            return evaluate_score(last_board, last_mask) * n;
        }
    }
    -1
}

fn apply_number_to_all(number: i32, boards: &[Vec<i32>], masks: &mut [Vec<bool>]) {
    masks
        .par_iter_mut()
        .zip(boards)
        .for_each(|(mask, board)| {
            board.iter().enumerate().for_each(|(idx, board_number)| {
                if board_number == &number {
                    mask[idx] = true;
                }
            });
            // thread::sleep(time::Duration::from_micros(500));
        })
}

fn is_board_winning(mask: &[bool]) -> bool {
    // thread::sleep(time::Duration::from_micros(500));
    (0..BOARD_WIDTH).any(|col| (0..BOARD_HEIGHT).all(|row| mask[(row * 5) + col]))
        || (0..BOARD_HEIGHT).any(|row| (0..BOARD_WIDTH).all(|col| mask[(row * 5) + col]))
}

fn evaluate_score(board: &[i32], hits: &[bool]) -> i32 {
    board
        .iter()
        .zip(hits.iter())
        .filter(|(_, h)| !**h)
        .map(|(n, _)| n)
        .sum()
}

fn parse_input(inp: &str) -> (Vec<i32>, Vec<Vec<i32>>) {
    let line_blocks = inp.split("\n\n").collect::<Vec<&str>>();

    let numbers = line_blocks
        .first()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let board_numbers = line_blocks[1..]
        .iter()
        .map(|t| {
            t.split('\n')
                .flat_map(|line| line.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (numbers, board_numbers)
}
