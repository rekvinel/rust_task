//1.
fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

//2.
// FILL in the blanks
fn main() {  
    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; // in two ways
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &s[0..5];
    assert_eq!(slice2, "hello");

    let slice3: &mut String = &mut s; 
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!")
}
 
 //3. 2

 //4. 
 fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10];//modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!")
}

//5.
// FILL in the blanks
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    // some bytes, in a vector
    let v = vec![104, 101, 108, 108, 111];

    // Turn a bytes vector into a String
    // We know these bytes are valid, so we'll use `unwrap()`.
    let s1 = String::from_utf8(v).unwrap();
    
    
    assert_eq!(s, s1);

    println!("Success!")
}

//6.
fn main() {
    let mut s = String::with_capacity(25);

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}
use std::mem;

//7.
fn main() {
    let story = String::from("Rust By Practice");

    // Prevent automatically dropping the String's data
    let mut story = mem::ManuallyDrop::new(story);

    let ptr = story.as_mut_ptr();
    let len = story.len();
    let capacity = story.capacity();

    // story has nineteen bytes
    assert_eq!(16, len);

    // We can re-build a String out of ptr, len, and capacity. This is all
    // unsafe because we are responsible for making sure the components are
    // valid:
    let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

    assert_eq!(*story, s);

    println!("Success!")
}