//1.

/* Annotate the lifetime of `i` and `borrow2` */

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses 
// both `borrow1` and `borrow2`. The duration of `borrow1` compared 
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3;                                             
    {                                                    
        let borrow1 = &i; 
        println!("borrow1: {}", borrow1); 
    } 
    {                                                    
        let borrow2 = &i; 
        println!("borrow2: {}", borrow2);               
    }                                                   
}   

//2.
//відсутній код


//3.

/* Make it work by adding proper lifetime annotation */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {}

//4.

// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn valid_output_owning() -> String { 
    String::from("foo")  // Return the owned String
}

fn main() {
    let s = valid_output_owning();
    println!("{}", s);
}

//5.

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
    let y: &i32 = &_x;
    println!("y is {}", y);
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}

//6.

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number    = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

//7.

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn main() {
    let var_a = 35;
    let var_b = NoCopyType {};
    let example = Example { a: &var_a, b: &var_b }; // Both `var_a` and `var_b` are valid here.

    println!("(Success!) {:?}", example);
}

//8.

#[derive(Debug)]
struct NoCopyType {}

#[derive(Debug)]
#[allow(dead_code)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType,
}

fn fix_me<'a, 'b>(foo: &'a Example<'a, 'b>) -> &'b NoCopyType {
    foo.b
}

fn main() {
    let no_copy = NoCopyType {};
    let example = Example { a: &1, b: &no_copy };
    fix_me(&example);
    println!("Success!");
}

//9.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {}

//10.

fn input(x: &i32) {
    println!("`input`: {}", x);
}

fn pass(x: &i32) -> &i32 { 
    x 
}

fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

struct Owner(i32);

impl Owner {

    fn add_one(&mut self) { self.0 += 1; }
    fn print(&self) {
        println!("`print`: {}", self.0);
    }
}

struct Person<'a> {
    age: u8,
    name: &'a str,
}

enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn main() {}
