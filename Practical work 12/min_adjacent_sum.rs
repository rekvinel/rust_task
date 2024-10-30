use std::time::{SystemTime, UNIX_EPOCH};

fn simple_random() -> u32 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    (since_epoch.as_secs() as u32) ^ (since_epoch.subsec_nanos())
}

fn gen_random_vector(n: usize) -> Vec<i32> {
    (0..n).map(|_| 10 + (simple_random() % 90) as i32).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_idx = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx = (i, i + 1);
        }
    }

    (min_idx.0, min_idx.1, min_sum)
}

fn print_vector_with_min_sum(data: &[i32]) {
    let (idx1, idx2, min_sum) = min_adjacent_sum(data);

    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2}. ", i);
    }
    println!();

    print!("data:   ");
    for num in data {
        print!("{:2}, ", num);
    }
    println!();

    print!("indexes: ");
    for _ in 0..idx1 {
        print!("    ");
    }
    print!("\\__ __/");
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}

fn main() {
    let data = gen_random_vector(35); //ввод чисел
    print_vector_with_min_sum(&data);
}