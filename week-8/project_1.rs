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
}