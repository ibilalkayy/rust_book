// Operators

// Operators are the symbols or the characters that perform some specific operation on data.

// Rust has 3 types of operators

// 1. Arithmetic operators
// 2. Logical operators
// 3. Comparison operators

// 1. Arithmetic operators are: +, -, *, /, %
// 2. Logical operators are: AND = &&, OR = ||, NOT = !
// 3. Comparison operators are: >, <, >=, <=, ==, !=

fn main() {
    // Arithmetic operators
    // println!("2 + 3 = {}", 2+3);
    // println!("5 - 3 = {}", 5-3);
    // println!("5 * 3 = {}", 5*3);
    // println!("10 / 2 = {}", 10/2);
    // println!("9 % 2 = {}", 9%2);

    // // Logical operators
    // // AND operator - It will look at both operands
    // println!("True AND True = {}", true && true);
    // println!("True AND False = {}",  true && false);
    // println!("False AND False = {}", false && false);

    // // OR operator - It will look at only one operand
    // println!("True OR True = {}", true || true);
    // println!("True OR False = {}",  true || false);
    // println!("False OR False = {}", false || false);

    // // NOT operater
    // println!("NOT True: {}", !true);
    // println!("NOT False: {}", !false);
    
    // Comparison operators
    let x = 10;
    let y = 11;
    println!("x is greater than y: {}", x>y);
    println!("x is smaller than y: {}", x<y);

    println!("x is greater than or equal to y: {}", x>=y);
    println!("x is smaller than or equal to y: {}", x<=y);

    println!("x is equal to y: {}", x==y);

    println!("x is not equal to y: {}", x!=y);
}