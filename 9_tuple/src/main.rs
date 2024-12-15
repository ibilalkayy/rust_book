fn main() {
    let tuple: (&str, i32, f32, bool) = ("Rust in 8 hours", 100, 4.99, true);
    println!(
        "String: {}\nInteger: {}\nFloat: {}\nBoolean: {}",
        tuple.0, tuple.1, tuple.2, tuple.3,
    );
}