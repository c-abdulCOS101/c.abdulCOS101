<<<<<<< HEAD
fn main(){
	//mutable array
	let mut colors = ["red", "purple","indigo","pink"];

	println!("\nOriginal array = {:?}", colors);

	//mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}" , sliced_colors);

    //change the value of the original slice at the first index}
    sliced_colors[1] = "yellow";

    println!("Changed slice = {:?}", sliced_colors);
=======
fn main(){
	//mutable array
	let mut colors = ["red", "purple","indigo","pink"];

	println!("\nOriginal array = {:?}", colors);

	//mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}" , sliced_colors);

    //change the value of the original slice at the first index}
    sliced_colors[1] = "yellow";

    println!("Changed slice = {:?}", sliced_colors);
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}