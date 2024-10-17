fn main() {
    const WIDTH: usize = 30;
    const HEIGHT: usize = 14;

    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 {
                output.push('*');
            } else if x == 0 || x == WIDTH - 1 {
                output.push('*');
            } else if x == y * 2 || x == WIDTH - y * 2 - 1 {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
