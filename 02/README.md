# 02

Exercise:

	fn main() {
		let name = "Michael";									// &str
		let x = println!("My name is {}", name);				// ()
		let this_year = 2019;									// i32
		let birth_year = 1985;									// i32
		let age = this_year - birth_year;						// i32
		let y = println!("I turned {} in {}", age, this_year);	// ()
		assert_eq!(x, y);
		let _z = println!("Thanks for chatting with me!");		// ()
	}

Output:

	My name is Michael
	I turned 34 in 2019
	Thanks for chatting with me!
