use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn possibilities(board: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let mut all = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 0..9 {
        all.retain(|d| *d != board[row][i]); // same col
        all.retain(|d| *d != board[i][col]); // same row
        all.retain(|d| *d != board[(row/3)*3 + i%3][(col/3)*3 + i/3]); // same square
    }
    all
}

fn process_board(solution_board: &mut Vec<Vec<u8>>, curr_i: usize,core_board: &Vec<Vec<u8>>,) -> bool {
    if curr_i == 81 {
        return true;
    }
    if core_board[curr_i/9][curr_i%9] != 0 {
        return process_board(solution_board, curr_i+1, core_board);
    }
    let possible_vals = possibilities(solution_board, curr_i/9, curr_i%9);
    for i in possible_vals {
        solution_board[curr_i/9][curr_i%9] = i;
        if process_board(solution_board, curr_i+1, core_board) {
            return true;
        }
    }
    solution_board[curr_i/9][curr_i%9] = 0;
    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("no input file was provided");
    }
    let file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(_) => panic!("invalid file path provided"),
    };

    let reader = BufReader::new(file);
    let core_board: Vec<Vec<u8>> = reader.lines().map(|line| {
        let line = line.unwrap();
        line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect();

    let mut solution_board: Vec<Vec<u8>> = core_board.clone();
    process_board(&mut solution_board, 0, &core_board);
    for line in solution_board {
        println!("{line:?}")
    }
}