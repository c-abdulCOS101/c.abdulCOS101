fn main(){

	let v  = vec!['C' , 'A' , 'S' , 'S' , 'A', 'N', 'D', 'R','A'];

	let mut input1 = String::new();

	println!("Enter an index value btw (0 -7)");
	std::io::stdin().read_line(&mut input1).expect("Failed to read input");
	//index is the non negative value which is smaller than the size of the vector
	let i:usize = input1.trim().parse().expect("Invalid input");


	//getting a value at given index value
	let ch: char = v[i];

	print!(" {} is the character for i [{}]\n", ch, i);
}