//1.
// Make it work with two ways
fn main() {
    let v = {
        let mut x = 1;
        x += 2
    };
 
    assert_eq!(v, ());

   println!("Success!");
}

//2.

fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
   println!("Success!");
}

//3.

fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}