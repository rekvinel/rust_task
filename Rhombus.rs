use std::io;

fn main() {
	let mut input = String::new();
	println!("Введіть кількість рядків:");
    
	io::stdin().read_line(&mut input).expect("Неправильно введено");
    
	let n: usize = input.trim().parse().expect("Введіть число");

	for i in 0..n {
    	for _ in 0..(n - i - 1) {
        	print!(" ");
    	}
    	for _ in 0..(2 * i + 1) {
        	print!("*");
    	}
    	println!();
	}

	for i in (0..n-1).rev() {
    	for _ in 0..(n - i - 1) {
        	print!(" ");
    	}
    	for _ in 0..(2 * i + 1) {
        	print!("*");
    	}
    	println!();
	}
}
