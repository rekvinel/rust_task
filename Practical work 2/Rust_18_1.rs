//1.

/* Make it work with least amount of changes*/
fn main() {
    let color = String::from("green");

    let print = || println!("`color`: {}", color);

    print();
    print();

    // `color` can be borrowed immutably again, because the closure only holds
    // an immutable reference to `color`. 
    let _reborrow = &color;

    println!("{}",color);
}

//2.

/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
fn main() {
    let count = 0;

    let inc = || {
        println!("`count`: {}", count);
    };

    inc();


    let _reborrow = &count; 

    inc();

    // The closure no longer needs to borrow `&mut count`. Therefore, it is
    // possible to reborrow without an error
    let _count_reborrowed = &mut {count}; 

    assert_eq!(count, 0);
}

//3.

/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
fn main() {
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        take(movable.clone());
    };

    consume();
    consume();
}

fn take<T>(_v: T) {}

//4.

fn main() {
    let example_closure_v1 = |x: String| -> String { x };
    let example_closure_v2 = |x: i32| -> i32 { x };
    
    let s = example_closure_v1(String::from("hello"));
    
    /* Make it work, only change the following line */
    let n = example_closure_v2(5);
    println!("String: {}", s);
    println!("Integer: {}", n);
}

//5.

