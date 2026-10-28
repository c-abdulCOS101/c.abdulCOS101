<<<<<<< HEAD
use std::fs::OpenOptions;
use std::io::Write;

fn main(){

	let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
	file.write_all("\nHello Class".as_bytes()).expect("write failed");
	file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
	println!("file append success");
=======
use std::fs::OpenOptions;
use std::io::Write;

fn main(){

	let mut file = OpenOptions::new().append(true).open("data.txt").expect("cannot open file");
	file.write_all("\nHello Class".as_bytes()).expect("write failed");
	file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
	println!("file append success");
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}