// method is a function that is in the context of structs, enums and traits

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) {
//         println!("{}", self.width * self.height);
//     }
// }

// fn main() {
//     let mut rectangle = Rectangle {
//         width: 45,
//         height: 2,
//     };
//     rectangle.area();
// }

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    fn description(&self, name: String) {
        match self {
            TrafficLight::Red => println!("Hey {}, you need to stop", name),
            TrafficLight::Green => println!("Hey {}, you need to go", name),
            TrafficLight::Yellow => println!("Hey {}, you need to be cautious", name),
        }
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    light.description("Bilal".to_string());
}