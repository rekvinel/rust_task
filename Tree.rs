fn main() {
    let h = 4;
    let shift = 6; 
    for j in 0..4 {
        for i in 0..h + j {
            let width = shift + h + i; 
            println!("{: >width$}", "*".repeat(2 * i + 1), width = width);
        }
        
    }
}