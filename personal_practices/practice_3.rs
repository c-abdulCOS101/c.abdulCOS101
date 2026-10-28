use std::io;

fn main(){

	let mut input_1 = String::new();
	let mut input_2 = String::new();
    let mut input_3 = String::new();


	println!("Welcome to the Area triangle calculator!\n");
	println!("Enter the first side :");


	io::stdin().read_line(&mut input_1).expect("Failed to read line");
	let a:i32 = input_1.trim().parse().expect("Not a valid integer");


	
	println!("Enter the second side :");
	io::stdin().read_line(&mut input_2).expect("Failed to read line");
	let b:i32 = input_2.trim().parse().expect("Not a valid integer");

	println!("Enter the third side: ");
	io::stdin().read_line(&mut input_3).expect("Failed to read line");
	let c:i32 = input_3.trim().parse().expect("Not a valid integer");

	let s:i32 = (a + b + c) / 2;
	let mut area:i32 = s* (s-a) * (s-b) * (s-c);
	area = area.isqrt();

	println!("Area of a triangle: {}", area );

}