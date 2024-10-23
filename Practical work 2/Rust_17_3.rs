//1.

struct DoubleRef<'r, 's, T> 
where 's: 'r,
{
    r: &'r T,
    s: &'s T,
}

fn main() {
    println!("Success!");
}

//2.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &'a str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let excerpt = ImportantExcerpt {
        part: "This is a very important part of the text.",
    };
    let announcement = "New announcement!";
    let result = excerpt.announce_and_return_part(announcement);
    
    println!("Result: {}", result);
    println!("Success!");
}

//3.

fn f<'a, 'b>(x: &'a i32, y: &'b mut &'a i32) {
    *y = x;
}

fn main() {
    let x = 10;
    let mut y = &0;
    
    f(&x, &mut y);
    println!("y: {}", y);
}

//4.

fn call_on_ref_zero<F>(f: F) 
where F: for<'a> Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    call_on_ref_zero(|x| println!("x: {}", x));
    println!("Success!");
}

//5.

fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    
    *ref1 += 1;
    
    let ref2 = &mut *ref1;
    
    *ref2 += 2;
    
    println!("{}", data);
}

//6.

struct Interface<'a> {
    manager: &'a mut Manager,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager {
    text: String,
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface(&mut self) -> Interface<'_> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: String::from("hello") },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    process_list(&list);
}

fn process_list(list: &List) {
    println!("{}", list.manager.text);
}