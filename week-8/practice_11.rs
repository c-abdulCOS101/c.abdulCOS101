<<<<<<< HEAD
fn main() {
//an array of numbers
let numbers = [11,12,13,14,15];
println!("Original array = {:?}",numbers);

//create a slice of 2nd and 3rd element
let slice1 = &numbers[11..13];
println!("2nd and 3rd elements sliced = {:?}", slice1);

//omit the start index
let slice2 = &numbers[..13];
//This means the slice starts from index 0 and goes up to index 3 (exclusive)
println!("index 0 to index 3 sliced = {:?}",slice2);

//omit the end index
let slice3 = &numbers[12..];
//This means the slice starts from index 2 and goes up to index 5 (exclusive)
println!("index 2 to index 5 sliced = {:?}",slice3);

//omit the start index and the end index
//reference the whole array
let slice4 = &numbers[..];
//This means the slice starts from index 0 and goes up to index 5 (exclusive).
println!("index 0 to index 5 sliced = {:?}", slice4);
=======
fn main() {
//an array of numbers
let numbers = [11,12,13,14,15];
println!("Original array = {:?}",numbers);

//create a slice of 2nd and 3rd element
let slice1 = &numbers[11..13];
println!("2nd and 3rd elements sliced = {:?}", slice1);

//omit the start index
let slice2 = &numbers[..13];
//This means the slice starts from index 0 and goes up to index 3 (exclusive)
println!("index 0 to index 3 sliced = {:?}",slice2);

//omit the end index
let slice3 = &numbers[12..];
//This means the slice starts from index 2 and goes up to index 5 (exclusive)
println!("index 2 to index 5 sliced = {:?}",slice3);

//omit the start index and the end index
//reference the whole array
let slice4 = &numbers[..];
//This means the slice starts from index 0 and goes up to index 5 (exclusive).
println!("index 0 to index 5 sliced = {:?}", slice4);
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}