fn is_palindrome(x: i32) -> bool {
	let s = x.to_string();
	s == s.chars().rev().collect::<String>()
}

fn main() {
	let num = 121;
	println!("{} є паліндромом: {}", num, is_palindrome(num));
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
    	let data = [
        	(123, false),
        	(121, true),
        	(1221, true),
    	];

    	data.iter().for_each(|(n, exp)| {
        	assert_eq!(is_palindrome(*n), *exp);
    	});
	}
}
