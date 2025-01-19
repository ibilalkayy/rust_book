// fn main() {
//     // let num: [u8; 4] = [1,2,3,4];
//     // for i in num {
//     //     println!("{}", i);
//     // }

//     for i in 1..=4 {
//         println!("{}", i);
//     }
// }

// fn main() {
//     let num: u8 = 4;
//     match num {
//         1 => println!("value is one"),
//         2 => println!("value is two"),
//         3..=4 => println!("the number is in the range of 3 to 4"),
//         _ => println!("no value is defined in this scope"),
//     }
// }

fn main() {
    let age: u8 = 5;
    match age {
        1 => println!("age is one"),
        age_num @ 3..=5 => {
            if age_num == 3 {
                println!("age is equal to three");
            } else {
                println!("age is in b/w 3 to 5: {}", age_num);
            }
        }
        _ => println!("nothing found"),
    }
}
