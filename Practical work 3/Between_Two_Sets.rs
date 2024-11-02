use std::io::{self, BufRead};

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;
    

    let max_a = *a.iter().max().unwrap();
    let min_b = *b.iter().min().unwrap();
    

    for x in max_a..=min_b {

        let is_factor_of_all_a = a.iter().all(|&ai| x % ai == 0);
        let is_divisible_by_all_b = b.iter().all(|&bi| bi % x == 0);
        
        if is_factor_of_all_a && is_divisible_by_all_b {
            count += 1;
        }
    }
    
    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let n = first_multiple_input[0];
    let m = first_multiple_input[1];

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    println!("{}", total);
}
