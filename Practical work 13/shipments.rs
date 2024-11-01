fn count_permutation(shipments: &[u32]) -> usize {
	let total: u32 = shipments.iter().sum();
	let n = shipments.len();
	if total % n as u32 != 0 { return 0; }

	let target = total / n as u32;
	let mut moves = 0;
	let mut balance: i32 = 0;

	for &shipment in shipments {
    	balance += shipment as i32 - target as i32;
    	moves += balance.abs();
	}

	moves as usize
}

fn main() {
	let shipments_list = vec![
    	vec![9, 3, 7, 2, 9],
	];

	for shipments in shipments_list {
    	let moves = count_permutation(&shipments);
    	let total: u32 = shipments.iter().sum();
    	let average = total / shipments.len() as u32;

    	println!("{:?}", shipments);
    	println!("total   = {}", total);
    	println!("average = {}", average);

    	let balance: Vec<i32> = shipments.iter().map(|&s| s as i32 - average as i32).collect();
   	 
    	for &b in &balance {
    	}
    	println!();
    	println!("answer = {}", moves);
	}
}