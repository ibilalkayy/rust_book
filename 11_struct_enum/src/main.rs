enum Color {
    Black,
    White,
    Red,
    Yellow,
    Brown,
}

struct Person {
    name: String,
    age: u8,
    gender: String,
    marital_status: String,
    color: Color,
}

fn main() {
    let person = Person {
        name: "Bilal Khan".to_string(),
        age: 40,
        gender: "Male".to_string(),
        marital_status: "Married".to_string(),
        color: Color::Black,
    };

    let result_of_color = match person.color {
        Color::Black => "Black color",
        Color::White => "White color",
        Color::Red => "Red color",
        Color::Yellow => "Yellow color",
        Color::Brown => "Brown color",
    };

    println!(
        "{}\n{}\n{}\n{}\n{}",
        person.name, person.age, person.gender, person.marital_status, result_of_color,
    );
}