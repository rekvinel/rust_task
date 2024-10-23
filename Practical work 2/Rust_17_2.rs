//1.

fn main() {
    let v = "hello"; // рядковий літерал має 'static тривалість життя
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}

//2.

#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG_STORAGE: Option<Config> = None;
static mut CONFIG: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    unsafe {
        CONFIG_STORAGE = Some(Config {
            a: "A".to_string(),
            b: "B".to_string(),
        });
        CONFIG_STORAGE.as_mut()
    }
}

fn main() {
    unsafe {
        CONFIG = init();

        println!("{:?}", CONFIG);
    }
}

//3.

fn main() {

    let static_string = "I'm in read-only memory";
    
    {

        println!("static_string: {}", static_string);

    }

    println!("static_string reference remains alive: {}", static_string);
}

//4.

// Make a constant with `'static` lifetime.
static NUM: i32 = 18;

// Returns a reference to `NUM` where its `'static`
// lifetime is coerced to that of the input argument.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Make an integer to use for `coerce_static`:
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`:
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

//5.

use std::fmt::Debug;

static I: i32 = 5;

fn print_it<T: Debug + 'static>(input: T) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it1(input: impl Debug + 'static) {
    println!("'static value passed in is: {:?}", input);
}

fn print_it2<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

fn main() {
    print_it(I);

    print_it(&I);

    print_it1(&I);

    print_it2(&I);
}

//6.

use std::fmt::Display;

fn main() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_e(&string);
    print_f(&string);
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}


fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t);
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t);
}