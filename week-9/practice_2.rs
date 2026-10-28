<<<<<<< HEAD
use std::io::Read;

fn main(){
	let mut file = std::fs::File::open("welcome_message.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
=======
use std::io::Read;

fn main(){
	let mut file = std::fs::File::open("welcome_message.txt").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}", contents);
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}