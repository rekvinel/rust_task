use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn timeConversion(s: &str) -> String {
    let period = &s[8..];
    let hours: i32 = s[0..2].parse().unwrap();
    let minutes = &s[3..5];
    let seconds = &s[6..8];
    
    let converted_hours = match period {
        "AM" => {
            if hours == 12 {
                0
            } else {
                hours
            }
        },
        "PM" => {
            if hours == 12 {
                12
            } else {
                hours + 12
            }
        },
        _ => hours,
    };
    
    format!("{:02}:{:02}:{:02}", converted_hours, minutes, seconds)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}