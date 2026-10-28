//declaring the structure

struct Laptopinfo {
    name: String,
    price: u32,
}

impl Laptopinfo {
    fn sum_cost(&self, quantity: u32) -> u32{
        self.price * quantity
    }
}
//initialiation
fn main() {
    println!("Good day kind sir/ma, Welcome to Alaba Electronics!");
        let hp = Laptopinfo {
        name : String::from("HP"),
        price : 650_000,
    };

    let ibm = Laptopinfo {
        name : String::from("IBM"),
        price : 755_000,
    };

    let toshiba = Laptopinfo {
        name : String::from("Toshiba"),
        price: 550_000
    };

    let dell = Laptopinfo {
        name: String::from("Dell"),
        price : 850_000,
    };

    let quantity:u32 = 3;

    let costof_hp = hp.sum_cost(quantity);
    let costof_ibm = ibm.sum_cost(quantity);
    let costof_toshiba = toshiba.sum_cost(quantity);
    let costof_dell = dell.sum_cost(quantity);
 
//to find the total sum
let sum = costof_hp + costof_ibm + costof_toshiba + costof_dell;
println!("The total cost of {} units of each laptop brand is {}",quantity, sum);
println!("Thank you for shopping at Alaba Electronics! , We hope to see you next time :D");
}
