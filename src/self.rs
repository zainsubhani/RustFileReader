struct Person {
    name: String,
    age: u8,
}

impl Person {
    // Immutable method
    fn greet(&self) {
        println!("Hello, my name is {}.", self.name);
    }

    // Mutable method
    fn have_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! I am now {} years old.", self.age);
    }

    // Method consuming the instance
    fn into_age(self) -> u8 {
        self.age // Returns the age and consumes the instance
    }
}

fn main() {
    let mut person = Person {
        name: String::from("Eve"),
        age: 29,
    };

    person.greet(); // Immutable borrow
    person.have_birthday(); // Mutable borrow
    let age = person.into_age(); // Ownership taken
    println!("Extracted age: {}", age);
    // `person` is no longer accessible here
}
