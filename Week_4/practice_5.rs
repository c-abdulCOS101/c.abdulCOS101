//Rust program to dtermine the height of an individual
//and then print if the person is tall, dwarf
//or an average height person

use std::io;

fn main()
{

	let mut input = String:: new();
	println!("\nEnter your Height (in centimeters): ");
	io::stdin().read_line(&mut input).expect("Not a valid string");
	let height: f32= input.trim().parse().expect("Not a valid number");

	if height >= 160.0 && height <= 180.0
	{
		println!("You are an average heighted person!");
	}
	else if height >180.0 && height <= 205.0
	{
		println!("You are a tall person!");
	}
	else if height < 160.0 && height > 100.0
	{
		println!("You are a dwarf!");
	}
	else {
		println!("You are of abnormal height!");
	}
}