use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    arr.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row[i] - row[arr.len() - 1 - i]
    }).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iter = stdin.lock().lines();
    let n = stdin_iter.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<Vec<i32>> = (0..n).map(|_| {
        stdin_iter.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }).collect();

    let result = diagonal_difference(&arr);
    writeln!(File::create(env::var("OUTPUT_PATH").unwrap()).unwrap(), "{}", result).ok();
}