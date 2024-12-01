// let slice = &array[0] // index is starting from zero
// in slice, the array is just a referrence to the original array

fn main() {
    let array: [i32; 5] = [5,6,7,8,9];
    // let slice = &array[1];
    // let slice = &array[1..];
    let slice = &array[0..];
    println!("{:?}", slice[4]);
}