// mod testing {
//     fn foo() {
//         println!("Foo is written");
//     }

//     pub fn foo_called() {
//         self::foo();
//     }
// }

// fn main() {
//     testing::foo_called();
// }


mod parent {
    fn parent_function() {
        println!("this is the parent function");
    }

    pub mod child {
        pub fn child_function() {
            super::parent_function();
            println!("this is the child function");
        }
    }
}

fn main() {
    parent::child::child_function();
}