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

/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: Fn(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
}


//6.

fn main() {
    let mut s = String::new();

    let update_string = |str: &str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

/* Fill in the blank */
fn exec<F>(mut f: F)
where
    F: FnMut(&str),
{
    f("hello");
}

//7.

fn apply<F>(f: F) 
where
    F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 
where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by mutable reference. Requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to be captured by value. Requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound.
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

//8.

/* Fill in the blank */
fn main() {
    let mut s = String::new();

    let update_string = |str: &str| { s.push_str(str); s.clone() };

    exec(update_string);
}

fn exec<F: FnMut(&str) -> String>(mut f: F) {
    println!("{}", f("hello"));
}

//9.

/* Implement `call_me` to make it work */
fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}

//10.

/* Fill in the blank using two approaches,
 and fix the error */
 fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;
    move |x| x + num
}

fn main() {
    let fn_plain = create_fn();
    println!("{}", fn_plain(1));
}

//11.

/* Fill in the blank and fix the error*/
fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1 {
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x + num)
    }
}

fn main() {
    let closure = factory(2);
    let result = closure(10);
    println!("Result: {}", result);
}