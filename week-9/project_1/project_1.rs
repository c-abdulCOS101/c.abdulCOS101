use std::io::Write;
fn main(){

	let categories = "Lager , Stout , Non-Alcoholic\n";
	let row_1 =  "33 Export , Legend , Maltina\n";
	let row_2 = "Goldberg , Williams , Malta Gold\n";
	let row_3 = "Gulder , , Fayrouz\n";
	let row_4 = "Heineken , , ,\n";
	let row_5 = "Star , , ,\n";

	let mut file = std::fs::File::create("nb.csv").expect("create failed");
    file.write_all(categories.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    file.write_all(row_4.as_bytes()).expect("write failed");
    file.write_all(row_5.as_bytes()).expect("write failed");
    println!("\nData written to file");
}