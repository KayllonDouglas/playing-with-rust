/*
 * This is a simple program that takes in a user's name and age,
 * instantiates a Person struct, and print them out.
 */

/*
 * This is the IO library that allows us to read user input.
 */
use std::io;

/*
 * This is a struct that represents a person.
 */
struct Person {
    name: String,
    age: u32,
}

/*
 * This is an implementation of the Person struct.
 */
impl Person {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

/*
 * This is the main function.
 * It takes in user input, instantiates a Person struct,
 * and prints out the name and age of the person.
 */
fn main() {
    let mut name = String::new();
    let mut age = String::new();

    println!("Enter your name: ");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter your age: ");

    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");

    let name = name.trim().to_string();
    let age: u32 = age.trim().parse().expect("Please type a number!");

    let person = Person::new(name, age);

    println!("Name: {}", person.get_name());
    println!("Age: {}", person.get_age());
}
