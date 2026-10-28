<<<<<<< HEAD
//** THE APS LEVEL CHECKER **//

use std::io;

fn main() {
    //Establishing the APS levels
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];
    //Establishing the roles under the APS levels
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];

println!("Enter the role you want to check for :");

   //creating a mutable string
     let mut exact_role = String::new();
     io::stdin().read_line(&mut exact_role).expect("Not a valid input");
     let exact_role = exact_role.trim();

     let mut matched_level = "Level not found";
     let mut found_department = "";


    // Going through office admin
    for i in 0..office_admin.len() {
        if office_admin[i] == exact_role {
            matched_level = levels[i];
            found_department = "office Admin";
        }
    }

    // Going through academic
    for i in 0..academic.len() {
        if academic[i] == exact_role {
            matched_level = levels[i];
            found_department = "academic";
        }
    }

    // Going through lawyer
    for i in 0..lawyer.len() {
        if lawyer[i] == exact_role {
            matched_level = levels[i];
            found_department = "lawyer";
        }
    }

    // Going through teacher
    for i in 0..teacher.len() {
        if teacher[i] == exact_role {
            matched_level = levels[i];
            found_department = "teacher";
        }
    }

     //**PRINTING THE FINAL RESULT**//
    if found_department != "" {
        println!("The postion for a {} in {} is: {}" , exact_role, found_department, matched_level);
    } else {
        println!("Role '{}' cannot be found in table.",exact_role);
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