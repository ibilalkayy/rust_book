// Array

// It's a kind of a variable that will store multiple values

// There are two ways to define an array in Rust

// 1. let array_one: [u32; 4] = [8; 4];
// 2. let array_two: [u32; 4] = [1,2,3,4];

fn main() {
    // First way to define an array
    // let mut array_one: [u32; 4] = [8; 4];
    // array_one[0] = 1;
    // array_one[1] = 2;
    // array_one[2] = 3;

    // println!("First: {}", array_one[0]);
    // println!("Second: {}", array_one[1]);
    // println!("Third: {}", array_one[2]);
    // println!("Fourth: {}", array_one[3]);

    // Second way to define an array
    let array_two: [u32; 3] = [11, 22, 33];
    println!("First: {}", array_two[0]);
    println!("Second: {}", array_two[1]);
    println!("Third: {}", array_two[2]);
}