// Defining the functions

// fn function_name(parameter: data_type) {
//     println!("{}", parameter);
// }

// fn myfun(data: String) {
//     println!("Here is the data: {}", data);
// }

// fn addition(x: i32, y: i32) {
//     println!("Addition result: {}", x+y);
// }

// fn subtraction(x: i32, y: i32) -> i32 {
//     return x-y;
// }

// fn boolean_fun(boolean: bool) -> bool {
//     return boolean;
// }

// fn main() {
//     let any_data: String = String::from("Hello World");
//     myfun(any_data);

//     let x_var: i32 = 45;
//     let y_var: i32 = 20;

//     addition(x_var, y_var);

//     let subtraction_result = subtraction(x_var, y_var);
//     println!("Subtraction result: {}", subtraction_result); 

//     let bool_var: bool = boolean_fun(true);
//     println!("Boolean result: {}", bool_var);
// }

// Variable binding

fn main() {
    let mut x: u32 = 34;
    println!("x: {}", x);

    x = x + 6;
    println!("x: {}", x);

    let (mut x, mut y): (i32, i32) = (100, 200);
    println!("x: {}", x);
    println!("y: {}", y);

    x = x + 6;
    println!("x: {}", x);

    y = y + 6;
    println!("y: {}", y);
}