//1.

fn main() {
	let s1 = "hello";
	let s = format!("{}, world!", s1);
	assert_eq!(s, "hello, world!");
}

//2.

fn main() {
   /* Fill in the blanks to make it print:
   Hello world, I am
   Sunface!
   */
   print!("hello world, ");
   print!("I am");
   println!("Sunface!");
}