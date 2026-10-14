fn main() {

	let z:(i32,bool,f64) = (23,true,12.8);
	print(z);

}

//pass the tuple as a parameter

fn print(x:(i32,bool,f64)) {

	println!("Inside print method");
	println!("{:?}",x);
}