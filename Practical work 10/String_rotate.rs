use std::io;

fn rotate(s: &str, n: isize) -> String {
	let len = s.len() as isize;
	if len == 0 {
    	return s.to_string();
	}

	let n = n.rem_euclid(len) as usize;
    
	format!("{}{}", &s[len as usize - n..], &s[..len as usize - n])
}

fn main() {
	let input = "abcdefgh";
	println!("Введіть число для зсуву:");

	let mut input_n = String::new();
	io::stdin()
    	.read_line(&mut input_n)
    	.expect("Не вдалося прочитати рядок");

	let n: isize = input_n.trim().parse().unwrap_or(0);

	let rotated = rotate(input, n);
	println!("Результат обертання на {}: {}", n, rotated);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rotate() {
    	let s = "abcdefgh";
    	let shifts = [
        	(0,  "abcdefgh"),
        	(8,  "abcdefgh"),
        	(-8, "abcdefgh"),
        	(1,  "habcdefg"),
        	(2,  "ghabcdef"),
        	(10, "ghabcdef"),
        	(-1, "bcdefgha"),
        	(-2, "cdefghab"),
        	(-10,"cdefghab"),
    	];

    	for (n, exp) in shifts.iter() {
        	assert_eq!(rotate(s, *n), *exp);
    	}
	}
}
