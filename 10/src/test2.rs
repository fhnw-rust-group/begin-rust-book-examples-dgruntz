#![allow(unused_variables)]
#![allow(dead_code)]

struct Person {
    name: &'static str,
    age: i32,
}

pub fn main() {
    let michael = Person {
        name: "Michael",
        age: 35,
    };
    println!("{} ist {} years old", michael.name, michael.age);
}

pub fn string() {
    let first_name = "Dominik".to_owned();
	let last_name = " Gruntz";
	let full_name:std::string::String = first_name + last_name;
	println!("Full name is {}", full_name);
}