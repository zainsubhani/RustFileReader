struct Person {
    name: String,
    age: u32,
    gender: Option<String>, // Optional field
}

fn struct_ex() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        gender: Some(String::from("Male")), // Correctly using Option<String>
    };

    // Accessing fields
    println!("Name: {}, Age: {}", person.name, person.age);

    // Checking if gender is present
    match &person.gender {
        Some(gender) => println!("Gender: {}", gender),
        None => println!("Gender: Not specified"),
    }
}

fn main() {
    struct_ex();
}
