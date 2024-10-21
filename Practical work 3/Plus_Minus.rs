use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let (mut positives, mut negatives, mut zeros) = (0, 0, 0);
    let total = arr.len() as f64;

    for &num in arr {
        match num {
            x if x > 0 => positives += 1,
            x if x < 0 => negatives += 1,
            _ => zeros += 1,
        }
    }

    println!("{:.6}", positives as f64 / total);
    println!("{:.6}", negatives as f64 / total);
    println!("{:.6}", zeros as f64 / total);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    plus_minus(&arr);
}