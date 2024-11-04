fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut frequency = [0; 5];

    for &bird in arr {
        frequency[(bird - 1) as usize] += 1;
    }

    let mut max_count = 0;
    let mut most_frequent_bird = 1;

    for (i, &count) in frequency.iter().enumerate() {
        if count > max_count || (count == max_count && ((i + 1) as i32) < most_frequent_bird) {
            max_count = count;
            most_frequent_bird = (i + 1) as i32;
        }
    }

    most_frequent_bird
}

fn main() {
    use std::io::{self, BufRead, Write};
    use std::fs::File;
    use std::env;

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _arr_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = migratoryBirds(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
