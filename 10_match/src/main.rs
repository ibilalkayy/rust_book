fn main() {
    let value: i32 = 2;
    // if value == 1 {
    //     println!("One");
    // } else if value == 2 {
    //     println!("Two");
    // } else if value == 3 {
    //     println!("Three");
    // } else {
    //     println!("Something else");
    // }

    let data = match value {
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Something else",
    };

    println!("{}", data);
}