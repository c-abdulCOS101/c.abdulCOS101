 // Rust program to calculate the area of a triangle given three sides
   use std::io;

   fn main()
   {
   	  let mut input1 = String::new();
   	  let mut input2 = String::new();
   	  let mut input3 = String::new();

   	  println!("Enter the first edge of triangle: ");
   	  io::stdin().read_line(&mut input1).expect("Not a valid string");
   	  let p:f32 = input1.trim().parse().expect("Not a valid number");

   	  println!("Enter the second edge of the triangle");
   	  io::stdin().read_line(&mut input2).expect("Not a valid string");
   	  let q:f32 = input2.trim().parse().expect("Not a valid number");

   	  println!("Enter the third edge of the triangle");
   	  io::stdin().read_line(&mut input3).expect("Not a valid string");
   	  let r:f32 = input3.trim().parse().expect("Not a valid number");

   	  let s:f32 = (p + q +r) /2.0 ;
   	  let mut area:f32 = s * (s - p) * (s - q) * (s - r);
   	  area = area.sqrt();

   	  println!("Area of triangle:{}", area );





   }