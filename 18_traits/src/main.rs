// a trait is a shared behaviour of mutliple objects.

enum Animal {
    Dog,
    Cat,
}

trait Sound {
    fn make_sound(&self);
}

impl Sound for Animal {
    fn make_sound(&self) {
        match self {
            Animal::Dog => println!("Woah Woah"),
            Animal::Cat => println!("Meow Meow"),
        }
    }
}

struct Data {
    age: u32,
}

trait Age {
    fn give_age(&self);
}

impl Age for Data {
    fn give_age(&self) {
        println!("Your age is: {}", self.age);
    }
}

fn main() {
    let animal = Animal::Cat;
    animal.make_sound();

    let data = Data {
        age: 32,
    };

    data.give_age();
}
