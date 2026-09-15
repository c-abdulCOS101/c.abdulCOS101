fn main(){
 let p:f64= 210_000.00;
 let n:i32= 3;
 let r:f64= 5.0;

 //depreciation
 let d = p * (1.0 - (r/100.0)).powi(n);
 println!("amount remaining is {}", d);



}