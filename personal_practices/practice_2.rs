use std::io;
fn main(){

	 println!("Welcome to the Sanctuary Spa!");
	 println! ("Enter your name :");
	let mut name = String::new();
	 io::stdin()
	.read_line(&mut name)
	.expect("Failed to read line");
	

    println!("Enter your age:");
	let mut age= String::new();
	io::stdin()
	.read_line(&mut age)
	.expect("Failed to read line");


	let age:i32 = age.trim().parse().expect("Not a valid integer");
	println!("Your age is :{}" , age);

    println!("Enter the name of your school");
	let mut school = String::new();
	io::stdin()
	.read_line(&mut school)
	.expect("Failed to read line");
	println!("Your school, {} is a nice school", school);

	println!("Enter your fathers age");

	let mut f_age= String::new();
	io::stdin()
	.read_line(&mut f_age)
	.expect("Failed to read line");

	let f_age:i32 = f_age.trim().parse().expect("Not a valid integer");
	println!("What!, your father is {}?  , that's so friggin old", f_age );
}