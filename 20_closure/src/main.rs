// Closure is a small and flexible function that you can define and use it inside the function


// syntax
// let function_name = |param, param| {body of function};

// fn add(num: u8) {
//     println!("{}", num+20);
// }

// fn main() {
//     // add(40);
//     let add = |num| num+20;
//     println!("{}", add(40));
// }


fn main() {
    let mut space = String::from("Hard disk space is 500");
    let mut unit = |u: &str| space.push_str(u);

    unit("GB");

    println!("{}", space);
}