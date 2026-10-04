use std::io;

fn main() {
    // Menu display
    println!("********** RESTAURANT MENU ************");
    println!("P = Poundo Yam / Edinkaiko Soup  - N3,200");
    println!("F = Fried Rice & Chicken          - N3,000");
    println!("A = Amala & Ewedu Soup            - N2,500");
    println!("E = Eba & Egusi Soup              - N2,000");
    println!("W = White Rice & Stew             - N2,500");
    println!("**************************************");

    // Read food type selection from customer
    println!("\nEnter the food type code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read line");
    let food_code = food_code.trim().to_uppercase();

    // Determine unit price using simple if/else statements
    let mut unit_price: f64 = 0.0;

    if food_code == "P" {
        unit_price = 3200.0;
    } else if food_code == "F" {
        unit_price = 3000.0;
    } else if food_code == "A" {
        unit_price = 2500.0;
    } else if food_code == "E" {
        unit_price = 2000.0;
    } else if food_code == "W" {
        unit_price = 2500.0;
    } else {
        println!("Invalid food selection.");
        return;
    }

    // Read quantity from customer
    println!("Enter quantity:");
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");

    // Convert string to number using simple parse + unwrap
    let quantity: f64 = quantity_input.trim().parse().unwrap();

    // Calculate initial total
    let mut total_charge = unit_price * quantity;
    println!("\nSubtotal: N{:.2}", total_charge);

    // Apply 5% discount if total is greater than N10,000
    if total_charge > 10000.0 {
        let discount = total_charge * 0.05;
        total_charge = total_charge - discount;
        println!("Discount applied (5%): -N{:.2}", discount);
    }

    println!("Total Charge: N{:.2}", total_charge);
}