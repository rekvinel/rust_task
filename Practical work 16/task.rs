fn next_permutation(data: &mut [i32]) -> bool {
    let mut k = None;
    for i in 0..data.len() - 1 {
        if data[i] < data[i + 1] {
            k = Some(i);
        }
    }

    if k.is_none() {
        return false;
    }
    let k = k.unwrap();

    let mut l = k + 1;
    for i in k + 1..data.len() {
        if data[k] < data[i] {
            l = i;
        }
    }

    data.swap(k, l);

    data[k + 1..].reverse();

    true
}

fn find_solutions() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut solutions = Vec::new();

    loop {
        let m = digits[0];
        let u = digits[1];
        let x = digits[2];
        let a = digits[3];
        let s = digits[4];
        let l = digits[5];
        let o = digits[6];
        let n = digits[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }

        if !next_permutation(&mut digits) {
            break;
        }
    }

    solutions
}

fn main() {
    let results = find_solutions();

    println!("Кількість рішень: {}", results.len());
    for solution in results {
        println!("Рішення: {:?}", solution);
    }
}
