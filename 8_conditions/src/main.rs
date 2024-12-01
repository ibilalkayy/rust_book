// if statement
// else if statement
// let if statement
// loop statement
// for statement
// while statement

fn main() {
    let age: u32 = 13;
    
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
    let result = if age == 0 {
        "Age cannot be zero".to_string()
    } else if age == 23 {
        "Age met the condition".to_string()
    } else {
        "Age is not meeting the condition".to_string()
    };

    println!("{}", result);
}