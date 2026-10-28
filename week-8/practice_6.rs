fn main() {
	//Create two vector
	let c = vec![11,12,13,14,15,16,17,18,];
	let a = vec![15,16,17,18,19,20,21,22,];

	//Use a for loop to add elements of the vector
	for index in 0..6{
		let sum = c[index] + a[index];
		println!("{:?}",sum);
	}

}