// if statement
// else if statement
// let if statement
// loop statement
// for statement
// while statement

fn main() {
    let mut num: u32 = 1;
    
    // .... if else satement
    // if age == 23 {
    //     println!("You're 23 years old and fit");
    // } else {
    //     println!("You're not 23 years old");
    // }

    // .... else if statement
    // if age == 0 {
    //     println!("Invalid age");
    // } else if age == 23 {
    //     println!("Valid age");
    // } else {
    //     println!("Age is crossed or below");
    // }

    // .... let if statement
    // let result = if age == 0 {
    //     "Age cannot be zero".to_string()
    // } else if age == 23 {
    //     "Age met the condition".to_string()
    // } else {
    //     "Age is not meeting the condition".to_string()
    // };

    // println!("{}", result);


    // Loop statement
    // loop {
    //     println!("Rust is {} hour(s)", num);
    //     if num == 8 {
    //         break;
    //     }
    //     num = num + 1;
    // }

    // while statement
    // while num <= 8 {
    //     println!("Rust in {num} hour(s)");
    //     num = num + 1;
    // }

    // for statement
    for i in 1..20 {
        num += i;
    }
    println!("Sum end result: {num}");
}