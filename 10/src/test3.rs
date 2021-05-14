#![allow(unused_variables)]
#![allow(dead_code)]

fn greet(name: &str) {
    println!("Hello {}", name);
}
pub fn main() {
    let first_name = "Dominik".to_owned();
	let last_name = " Gruntz";
	let full_name = first_name + last_name;
    greet(&full_name);
    greet(&full_name);

    let snoyman: &str = " Snoyman";
    let michael: String = "Michael".to_owned() + snoyman;
    let miriam: String = "Miriam".to_owned() + snoyman;
    let alice: &str = "Alice Smith";

    greet(&michael);
    greet(&miriam);
    greet(alice);
}