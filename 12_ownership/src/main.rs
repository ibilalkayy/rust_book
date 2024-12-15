fn main() {
    // // x variable is the owner of Hello world string
    // let x: String = String::from("Hello world!");
    
    // // Before transferring the ownership
    // println!("Value of x: {}", x);

    // // After transferring the ownership
    // let y = x;
    // println!("Value of y: {}", y);

    // // It will give the error
    // println!("Value of x: {}", x);

    let a_int: u8 = 45;
    println!("Value of a int: {}", a_int);

    let b_int: u8 = a_int;

    println!("Value of b int: {}", b_int);

    println!("Value of a int again: {}", a_int);
}
