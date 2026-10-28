fn main(){
	//Name vector
	let name= vec!["Cassandra","Jennifer","Olu","DDD","CCC","FOF","JIJ","KIL"];

	let age = vec![18, 29, 82 , 34, 48 , 87 , 67 , 91];

	println!("\nAge allocation");

	for i in 0..age.len()
	{
		print!("{} is {} years old]n", name[i],age[i]);
	}

}