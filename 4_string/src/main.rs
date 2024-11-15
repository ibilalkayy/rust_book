fn main() {
    // there are three ways to define a string

    let first_way: &str = "Hello World";
    println!("{}", first_way);


    let mut second_way: String = "Bye World".to_string();
    println!("{}", second_way);

    second_way.push('!');
    println!("{}", second_way);

    let mut third_way: String = String::from(first_way);
    println!("{}", third_way);

    third_way.push('!');
    println!("{}", third_way);
}