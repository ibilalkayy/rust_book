// Data types

// fn main() {
//     // let variable_name: data_type = value;

//     let integer: i8 = -127; // 2*16 = 65536, -2^7 to 2^7 - 1 = -128 to 127
//     println!("{}", integer);

//     let integer_two: usize = 34;
//     let integer_three: isize = 43;

//     println!("Usize: {}", integer_two);
//     println!("Isize: {}", integer_three);

//     let size_of_usize = std::mem::size_of::<usize>();
//     println!("{}", size_of_usize * 8);

//     let size_of_isize = std::mem::size_of::<isize>();
//     println!("{}", size_of_isize * 8);

//     let floating_point: f32 = 43.6546345;
//     println!("{:.2}", floating_point);

//     // -3.4 * 10^38 to 3.4 * 10^38;

//     let string_var = "this is the string";
//     println!("{}", string_var);

//     let bool_var: bool = true;
//     println!("{}", bool_var);

//     let array: [u8; 4] = [1,2,3,4];
//     println!("{}", array[3]);

//     let slice = &array[0..2];
//     println!("{:?}", slice);

//     let tuple = (1, "string", true);
//     println!("{}", tuple.0);

//     let character = 'h'
//     println!("{}", character);
// }


// Constants

// const VARIABLE_NAME: u8 = 148;

// fn main() {
//     println!("{}", VARIABLE_NAME);
// }

// Data type conversion

fn main() {
    let float: f32 = 324.54353;
    let integer: u32 = float as u32;

    println!("{}", integer);
}