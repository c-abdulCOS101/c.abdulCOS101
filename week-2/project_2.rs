fn main (){
   let t_amount:i32= 450_000;
   let m_amount:i32= 1_500_000;
   let h_amount:i32= 750_000;
   let d_amount:i32= 2_850_000;
   let a_amount:i32= 250_000;
   let q1:i32 = 2;
   let q2:i32 = 1;
   let q3:i32 = 3;
   let q4:i32 = 3;
   let q5:i32 = 1;


   //total quantity
   let q = q1 + q2+ q3 + q4 +q5;

   //total sum
   let sum = (t_amount * q1) + (m_amount * q2) + (h_amount * q3) + (d_amount * q4) +(a_amount * q5);
   println!("The total sum is {}", sum); 

   //average
   let average = sum / q;
   println!("the average is {}", average);
}