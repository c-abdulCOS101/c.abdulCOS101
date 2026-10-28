
use std::io::Write;
 
//headers
fn main(){
	let column_headers =vec!["commissioner", "ministry", "zone"];

	let commissioner: [&str; 5] = ["Aigbogun Alamba Daudu", "Murtala Afeez Bendu" , "Okorocha Calistus Ogbone", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
	let ministry: [&str; 5] = ["Internal Affairs" , "Justice", "Defense", "Power&Steel", "Petroleum"];
	let zone: [&str; 5] = ["South West", "North East", "South South", "South West", "South East"];
//rows including index
let efcc_1 = vec![commissioner[0] , ministry[0] , zone[0]];
let efcc_2 = vec![commissioner[1] , ministry[1] , zone[1]];
let efcc_3 = vec![commissioner[2] , ministry[2] , zone[2]];
let efcc_4 = vec![commissioner[3] , ministry[3] , zone[3]];
let efcc_5 = vec![commissioner[4] , ministry[4] , zone[4]];
//~~~~time to create the file :) ~~~~
let mut file = std::fs::File::create("efcc merge.csv").expect("create failed");
//now, we are going to use the for loop and index ;)
for i in 0..column_headers.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(column_headers[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");
for i in 0..efcc_1.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(efcc_1[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");

for i in 0..efcc_2.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(efcc_2[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");

for i in 0..efcc_3.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(efcc_3[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");

for i in 0..efcc_4.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(efcc_4[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");

for i in 0..efcc_5.len(){
	if i > 0 {file.write_all(",".as_bytes()).expect("write failed"); }
	file.write_all(efcc_5[i].as_bytes()).expect("write failed");
}
file.write_all("\n".as_bytes()).expect("failed to write");

println!("The EFCC merge file has been created and written, you're welcome!!");
}