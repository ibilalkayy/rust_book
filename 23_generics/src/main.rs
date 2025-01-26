// // Generics is the feature of Rust that will take multiple data types in one place

// // fn data<T>(value: T) -> T {
// //     return value;
// // }

// // fn main() {
// //     let string: String = String::from("Hello World");
// //     let integer: i8 = 45;
// //     let float: f32 = 45.5;

// //     println!("String: {}", data(string));
// //     println!("Integer: {}", data(integer));
// //     println!("Float: {}", data(float));
// // }

// struct Holder<T> {
//     value: T,
// }

// impl<T> Holder<T> {
//     fn new(value: T) -> Self {
//         Holder{value}
//     }
// }

// fn main() {
//     let string = Holder::new("Hello world".to_string());
//     let integer = Holder::new(45);
//     let float = Holder::new(45.5);
//     println!("{}\n{}\n{}", string.value, integer.value, float.value);
// }