const N: usize = 5;
fn main() {
    let mut diamond = String::new();
    for i in 0..(2 * N - 1) {
        let row = if i < N { i } else { 2 * N - 2 - i };
        diamond.push_str(&" ".repeat(N - 1 - row));
        diamond.push_str(&"".repeat(2 row + 1));
        diamond.push('\n');
    }
    print!("{}", diamond);
}
