// Drop method will remove or clean up the resources from the memory even if you're present inside the scope

// use std::mem;

// fn main() {
//     {
//         let value: String = String::from("Hello world!");
//         println!("Inside the scope: {}", value);
        
//         mem::drop(value.clone());

//         println!("Inside the scope: {}", value);

//     }
//     // println!("Outside the scope: {}", value);
// }

use std::mem;

#[derive(Debug, Clone)]
struct Resource {
    name: String,
}

// this is the drop method for resource
impl Drop for Resource {
    fn drop(&mut self) {
        println!("Defining resource: {}", self.name);
    } 
}

fn main() {
    {
        let res = Resource {name: "Hello World".to_string()};
        println!("Inside the scope Resource: {:?}", res);

        mem::drop(res.clone());

        println!("Inside the scope Resource after drop: {:?}", res);
    }

    // println!("Outside the scope Resource: {:?}", res);
}