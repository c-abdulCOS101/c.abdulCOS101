<<<<<<< HEAD
use std::io::Write;
fn main(){

	let announce = "Week 9 - Rust File Input & Output\n";
	let dept=  "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
    	.as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file");
=======
use std::io::Write;
fn main(){

	let announce = "Week 9 - Rust File Input & Output\n";
	let dept=  "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
    	.as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\nData written to file");
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}