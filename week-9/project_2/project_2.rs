use std::io::Write;

fn main(){
	//smis header
    let header = vec!["Student Name, Matric Number , Department, Level"];

    //smis rows
    let smis_1= vec!["Oluchi Mordi" , "ACC10211111", "Accounting", "300"];
	let smis_2 = vec!["Adams Aliyu" ,"ECO10110101","Economics","100"];
	let smis_3 = vec!["Shania Bolade","CSC10328828","Computer","200"];
	let smis_4= vec!["Adekunle Gold", "EEE11020202", "Electrical", "200"];
    let smis_5 = vec!["Bianca Edemoh", "MEE10202001", "Mechanical","100"];

    let mut file = std::fs::File::create("pausmis.csv").expect("create failed");
    file.write_all("PAU-SMIS\n".as_bytes()).expect("write failed");
    
    for i in 0..header.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(header[i].as_bytes()).expect("write failed");
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..smis_1.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(smis_1[i].as_bytes()).expect("write failed");
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..smis_2.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(smis_2[i].as_bytes()).expect("write failed");
    }
    
    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..smis_3.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(smis_3[i].as_bytes()).expect("write failed");
    }
    file.write_all("\n".as_bytes()).expect("write failed");

    for i in 0..smis_4.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(smis_4[i].as_bytes()).expect("write failed");
    }

    file.write_all("\n".as_bytes()).expect("write failed");
    
    for i in 0..smis_5.len(){
    	if i > 0 { file.write_all(",".as_bytes()).expect("write failed");}
    	file.write_all(smis_5[i].as_bytes()).expect("write failed");
    }

    file.write_all("\n".as_bytes()).expect("write failed");

    println!("The PAU SMIS file has been wriiten");
}