mod first {
    pub fn first_fn() {
        println!("This is the first module printed");
    }
}

mod second_module {
    pub struct Data {
        pub firstname: String,
        pub lastname: String,
    }

    pub fn second_fn() {
        let data = Data {
            firstname: "Bilal".to_string(),
            lastname: "Khan".to_string(),
        };
        println!("Firstname: {}\nLastname: {}", data.firstname, data.lastname);
    }
}

mod parent_module {
    pub fn a() {
        println!("parent module is printed");
    }

    pub mod child_module {
        pub fn b() {
            println!("child module is printed");
        }
    }
}


fn main() {
    first::first_fn();
    second_module::second_fn();

    parent_module::a();
    parent_module::child_module::b();
}