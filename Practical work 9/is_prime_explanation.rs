fn is_prime(n: u64) -> bool {
	if n <= 1 || (n > 2 && n % 2 == 0) {
    	return false;
	}
	let limit =(n as f64).sqrt() as u64;
	let range =3..limit;
	range.filter(|i| n % i == 0).count() == 0
}

fn main(){
	let number = 16;
	println!("Is {} prime? {}",number,is_prime(number));
}