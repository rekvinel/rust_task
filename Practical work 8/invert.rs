fn swap_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_lowercase().collect::<String>()
            }
        })
        .collect()
}

fn main() {
    let input = "HellO WOrLd!!";
    println!("Початкове: {}", input);
    println!("Змінено: {}", swap_case(input));
}
