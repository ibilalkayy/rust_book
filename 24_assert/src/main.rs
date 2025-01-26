// assert is a macro
// assert is used for debugging and testing 
// whenever you want to write the expression and based on that expression evaluates the true and false condition

fn main() {
    let x = 100; let y = 200;
    assert!(x + y == 300, "x plus y is not equal to 300");
    println!("The program is running normally by giving: {}", x + y);
}