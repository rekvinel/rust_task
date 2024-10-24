//1.
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
 }

 //2.
fn main() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success!");
}

//3.
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}

//4.
// Fix error
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

//5.
fn main() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}

//6.
fn main() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

//7.
// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

//8.
fn main() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}

//9.
// This code has no errors!
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}

//10.
// Comment one line to make it work
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
   //println!("{}",r1);
}

//11.
fn main() {
    let mut s = String::from("hello, ");

    {
        let r1 = &mut s;
        println!("{}", r1);  // Використання r1
    }  // r1 виходить з області видимості тут

    let r2 = &mut s;  // Тепер можна створити r2
    println!("{}", r2);
}
