use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn possiblitys(board: &Vec<Vec<u8>>, row: usize, col: usize) -> Vec<u8> {
    let square_row = row/3*3;
    let square_col = col/3*3;
    let mut all = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in 0..9 {
        all.retain(|d| *d != board[row][i]); // same col
        all.retain(|d| *d != board[i][col]); // same row
        all.retain(|d| *d != board[square_row + i%3][square_col + i/3]); // same square
    }
    all
}

fn recursive_solve(solving: &mut Vec<Vec<u8>>, i: usize, core: &Vec<Vec<u8>>,) -> bool {
    if i == 81 {
        return true;
    }
    if core[i/9][i%9] != 0 {
        return recursive_solve(solving, i+1, core);
    }
    let possibles = possiblitys(solving, i/9, i%9);
    for trial in possibles {
        solving[i/9][i%9] = trial;
        if recursive_solve(solving, i+1, core) {
            return true;
        }
    }
    solving[i/9][i%9] = 0;
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
    let core: Vec<Vec<u8>> = reader.lines().map(|line| {
        let line = line.unwrap();
        line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
    }).collect();

    let mut solved: Vec<Vec<u8>> = core.clone();
    recursive_solve(&mut solved, 0, &core);
    for (i, line) in solved.iter().enumerate() {
        for (j, digit) in line.iter().enumerate() {
            print!("{digit}");
            if j % 3 == 2 && j != 8 {
                print!("|")
            }
        }
        println!("");
        if i % 3 == 2 && i != 8 {
            println!("---+---+---");
        }
    }
}