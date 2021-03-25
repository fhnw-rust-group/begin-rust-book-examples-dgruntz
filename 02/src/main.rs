fn _main() {
    let x: () = ();

    let a = {5};
    let _e: () = if a == 15 { 1; };

    println!("Value of x == (): {}", x == ());
}

fn main() {
    let name: &str = "Michael";									// &str
    let x: () = println!("My name is {}", name);				// ()
    let this_year: i32 = 2019;									// i32
    let birth_year: i32 = 1985;									// i32
    let age: i32 = this_year - birth_year;						// i32
    let y: () = println!("I turned {} in {}", age, this_year);	// ()
    assert_eq!(x, y);
    let _z: () = println!("Thanks for chatting with me!");		// ()
}

