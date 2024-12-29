// - mod module
// - use bring that module into the current scope

mod greetings;
           
           //module //submodule //function/struct/enum
use crate::greetings::happy::happy;
use crate::greetings::data::Data;

fn main() {
    println!("Hello, world!");
    happy();
    let var = Data {
        name: String::from("Bilal Khan"),
        age: 42,
    };
    println!("{:?}", var);
}
