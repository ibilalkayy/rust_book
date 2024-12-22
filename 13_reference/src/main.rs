// fn main() {
//     // x variable contains the data and owns it
//     let x = 10;

//     // y is reference of the data
//     let y = &x;
//     println!("reference of data: {}", y);
//     println!("reference of data: {}", y);
//     println!("reference of data: {}", y);
//     println!("reference of data: {}", y);

//     // z is the dereference of data
//     let z = *y;
//     println!("dereference of data: {}", z);
// }



// you cannot change the reference of data. instad you can change the dererfence in order to modify the actual variable
// fn main() {
//     let mut x = 10;
//     let y = &mut x;

//     *y += 1;
//     println!("changing the dereference of data: {}", y);
// }

// you can only mutate the reference one time
fn main() {
    let mut x = 10;
    let y = &mut x;
    let z = &mut x;

    println!("{}", y);
}