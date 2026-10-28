<<<<<<< HEAD
use std::io;

fn trapezium_area(height: f64, base1: f64, base2: f64) {
    let area = height / 2.0 * (base1 + base2);
    println!("Area of the trapezium = {:.2}", area);
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) {
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of the rhombus = {:.2}", area);
}

fn parallelogram_area(base: f64, altitude: f64) {
    let area = base * altitude;
    println!("Area of the parallelogram = {:.2}", area);
}

fn cube_surface_area(side: f64) {
    let area = 6.0 * side * side;
    println!("Surface area of the cube = {:.2}", area);
}

fn cylinder_volume(radius: f64, height: f64) {
    let volume = 3.141592653589793 * radius * radius * height;
    println!("Volume of the cylinder = {:.2}", volume);
}

fn show_menu() {
    println!();
    println!("===== SHAPE CALC =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("0. Quit");
    println!("======================");
}

fn main() {
    let get_text = |label: &str| {
        println!("{}", label);
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Could not read input");
        text.trim().to_string()
    };

    let get_number = |label: &str| {
        loop {
            let text = get_text(label);

            let value = text.parse::<f64>().into_iter().sum::<f64>();

            if value > 0.0 {
                return value;
            }
            println!("  Enter a valid number greater than 0.");
        }
    };

    loop {
        show_menu();
        let choice = get_text("Enter your choice (0-5):");

        if choice == "1" {
            let height = get_number("Enter the height:");
            let base1 = get_number("Enter base 1:");
            let base2 = get_number("Enter base 2:");
            trapezium_area(height, base1, base2);
        } else if choice == "2" {
            let d1 = get_number("Enter diagonal 1:");
            let d2 = get_number("Enter diagonal 2:");
            rhombus_area(d1, d2);
        } else if choice == "3" {
            let base = get_number("Enter the base:");
            let altitude = get_number("Enter the altitude:");
            parallelogram_area(base, altitude);
        } else if choice == "4" {
            let side = get_number("Enter the side length:");
            cube_surface_area(side);
        } else if choice == "5" {
            let radius = get_number("Enter the radius:");
            let height = get_number("Enter the height:");
            cylinder_volume(radius, height);
        } else if choice == "0" {
            println!("Goodbye!");
            break;
        } else {
            println!("Invalid choice. Enter a number from 0 to 5.");
        }
    }
=======
use std::io;

fn trapezium_area(height: f64, base1: f64, base2: f64) {
    let area = height / 2.0 * (base1 + base2);
    println!("Area of the trapezium = {:.2}", area);
}

fn rhombus_area(diagonal1: f64, diagonal2: f64) {
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Area of the rhombus = {:.2}", area);
}

fn parallelogram_area(base: f64, altitude: f64) {
    let area = base * altitude;
    println!("Area of the parallelogram = {:.2}", area);
}

fn cube_surface_area(side: f64) {
    let area = 6.0 * side * side;
    println!("Surface area of the cube = {:.2}", area);
}

fn cylinder_volume(radius: f64, height: f64) {
    let volume = 3.141592653589793 * radius * radius * height;
    println!("Volume of the cylinder = {:.2}", volume);
}

fn show_menu() {
    println!();
    println!("===== SHAPE CALC =====");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("0. Quit");
    println!("======================");
}

fn main() {
    let get_text = |label: &str| {
        println!("{}", label);
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Could not read input");
        text.trim().to_string()
    };

    let get_number = |label: &str| {
        loop {
            let text = get_text(label);

            let value = text.parse::<f64>().into_iter().sum::<f64>();

            if value > 0.0 {
                return value;
            }
            println!("  Enter a valid number greater than 0.");
        }
    };

    loop {
        show_menu();
        let choice = get_text("Enter your choice (0-5):");

        if choice == "1" {
            let height = get_number("Enter the height:");
            let base1 = get_number("Enter base 1:");
            let base2 = get_number("Enter base 2:");
            trapezium_area(height, base1, base2);
        } else if choice == "2" {
            let d1 = get_number("Enter diagonal 1:");
            let d2 = get_number("Enter diagonal 2:");
            rhombus_area(d1, d2);
        } else if choice == "3" {
            let base = get_number("Enter the base:");
            let altitude = get_number("Enter the altitude:");
            parallelogram_area(base, altitude);
        } else if choice == "4" {
            let side = get_number("Enter the side length:");
            cube_surface_area(side);
        } else if choice == "5" {
            let radius = get_number("Enter the radius:");
            let height = get_number("Enter the height:");
            cylinder_volume(radius, height);
        } else if choice == "0" {
            println!("Goodbye!");
            break;
        } else {
            println!("Invalid choice. Enter a number from 0 to 5.");
        }
    }
>>>>>>> 76ae95a9528d66b0392fe386ccd3bd6b966d10cc
}