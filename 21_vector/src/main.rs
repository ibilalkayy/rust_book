// What is Vector in Rust?
// Vector in rust is the dynamic array
// Dynamic array is the array whose value is not fixed and it's size can be changed
// Array is stored in the stack memory and the vector is going to be stored in the heap memory

// Vector can be defined in three ways

// first way

// fn main() {
//     let mut data: Vec<u32> = vec![100, 200, 300, 400];
//     data.push(500);
//     println!("{:?}", data);
// }

// second way
// fn main() {
//     let mut data: Vec<String> = Vec::new();
//     data.push("Bilal".to_string());
//     data.push("Khan".to_string());
//     println!("{:?}", data);
// }

// third way
fn main() {            // default value dynamic length
    let mut data: Vec<u32> = vec![8; 1];
    data.push(9);
    data.push(10);

    for i in &data {
        println!("{:?}", i);
    }

    data.pop();
    data.pop();

    for i in data {
        println!("{:?}", i);
    }
}