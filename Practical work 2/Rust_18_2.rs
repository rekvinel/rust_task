//1.

fn main() {
    let arr = [0; 10];
    for value in arr.iter() {
        println!("{}", value);
    }
}

//2.

fn main() {
    let mut v = Vec::new();
    for n in 0..100 {
        v.push(n);
    }

    assert_eq!(v.len(), 100);
}

//3.

fn main() {
    let v1 = vec![1, 2];
    let mut iter = v1.iter();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), None);
}

//4.

/* Make it work */
fn main() {
    let arr = vec![0; 10];
    for i in &arr {
        println!("{}", i);
    }

    println!("{:?}",arr);
}

//5.

fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

//6.

fn main() {
    let mut values = vec![1, 2, 3];
    let mut values_iter = values.iter_mut();

    if let Some(v) = values_iter.next() {
        *v = 0;
    }

    assert_eq!(values, vec![0, 2, 3]);
}

//7.

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;
    
    /* Implement next method */
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut fib = fibonacci();
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(1));
    assert_eq!(fib.next(), Some(2));
    assert_eq!(fib.next(), Some(3));
    assert_eq!(fib.next(), Some(5));
}


//8.


fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    println!("{:?}", v1);
}


//9.

/* Make it work */
use std::collections::HashMap;

fn main() {
    let names = [("sunface", 18), ("sunfei", 18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}", folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().cloned().collect();

    assert_eq!(v2, vec![1, 2, 3]);
}

//10.

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

//11.

use std::collections::HashMap;

fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();

    println!("{:?}", folks);
}

//12.

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}