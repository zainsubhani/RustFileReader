#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: u32,
}

impl Person {
    // Method to calculate and return the full name
    fn full_name(&self) -> String {
        format!(
            "{} {}{} {} {}",
            self.first_name, self.last_name, self.age, self.email, self.phone_number
        )
    }
}
fn main() {
    // Create a Person instance
    let person = Person {
        first_name: String::from("Alice"),
        last_name: String::from("Smith"),
        age: 30,
        email: "zain@gmail.com".to_string(),
        phone_number: 090078601,
    };

    // Call the full_name method and print the result
    let full_name = person.full_name();
    println!("Full Name: {}", full_name);
}
