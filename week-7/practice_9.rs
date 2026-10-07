// The iter() function fetches values of all elements in an array.[cite: 9]

fn main(){[cite: 9]

    let arr:[i32;4] = [10,20,30,40];[cite: 9]
    println!("array is {:?}",arr);[cite: 9]
    println!("array size is :{}",arr.len());[cite: 9]

    for val in arr.iter(){[cite: 9]
        println!("value is :{}",val);[cite: 9]
    }[cite: 9]
}[cite: 9]