#![allow(unused_variables)]
#![allow(dead_code)]

mod test1;
mod test2;
mod test3;

fn main() {
	test1::main();
	// test2::main();
	// test2::string();
	// test3::main();

	// let michael: String = add_snoyman("Michael");
	// let miriam: String = add_snoyman("Miriam");
	// let john: &'static str = "John Doe";
	// say_hi(&michael);
	// say_hi(&miriam);
	// say_hi(john);
}

fn add_snoyman(name: &str) -> String {
	name.to_owned() + " Snoyman"
}

fn say_hi(name: &str) {
	println!("Hi {}", name);
}

