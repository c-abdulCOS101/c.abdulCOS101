fn main() {
    //listing put the candidates and their level of experience
    let candidates = vec![("Divine" , 23),("Shadaii" , 21),("Juwon", 5),("Cassandra", 8) , ("Messiah",16) , ("Valerie", 1)];

    // Assume the first candidate has the highest experience initially
    let mut best_name = candidates[0].0;
    let mut max_years = candidates[0].1;

    // Loop through the vector starting from the second item
    let mut i = 1;
    while i < candidates.len() {
        let name = candidates[i].0;
        let years = candidates[i].1;

        // If the current candidate has more experience, update our records
        if years > max_years {
            max_years = years;
            best_name = name;
        }
        i += 1;
    }

    // Print the final result
    println!("The person with the highest years of experience is {} with {} years.", best_name, max_years);