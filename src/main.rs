use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn possibles(board: &Vec<u8>, i: usize) -> Vec<u8> {
    let row_start = i/9*9;
    let col_start = i%9;
    let square_row = i/27*27;
    let square_col = ((i%9)/3)*3;
    let mut all = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for j in 0..9 {
        all.retain(|d| *d != board[col_start+j*9]); // same col
        all.retain(|d| *d != board[row_start+j]); // same row
        all.retain(|d| *d != board[square_row+(j/3)*9 + square_col+j%3]); // same square
    }
    all
}

fn recursive_solve(solving: &mut Vec<u8>, i: usize, core: &Vec<u8>,) -> bool {
    if i == 81 {
        return true;
    }
    if core[i] != 0 {
        return recursive_solve(solving, i+1, core);
    }
    let possibles = possibles(solving, i);
    for trial in possibles {
        solving[i] = trial;
        if recursive_solve(solving, i+1, core) {
            return true;
        }
    }
    solving[i] = 0;
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
    let core: Vec<u8> = reader.lines().map(|line| {
        let line = line.unwrap();
        if line.len() != 9 {
            panic!("invalid row length");
        }
        line.trim().chars().map(|c| 
            match c.to_digit(10) {
                Some(d) => d as u8,
                None => panic!("invalid character"),
            }).collect::<Vec<u8>>()
    }).flatten().collect();

    if core.len() != 81 {
        panic!("invalid number of rows provided in file");
    }

    let mut solved: Vec<u8> = core.clone();
    recursive_solve(&mut solved, 0, &core);
    for (i, digit) in solved.iter().enumerate() {
        print!("{digit}");
        if i % 3 == 2 && i % 9 != 8 {
            print!("|");
        }
        if i % 9 == 8 {
            println!("");
        }
        if i % 27 == 26 && i != 80 {
            println!("---+---+---");
        }
    }
}