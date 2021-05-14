#![allow(unused_variables)]
#![allow(dead_code)]

pub mod util;

use std::ptr;

fn get_value() -> &'static str {
    "Hello"
}

pub fn main() {
	let s1: &str = "Hello";
	let s2: &str = "Hello";
	let s3: &str = get_value();
    let s4: &str = util::get_value();

    println!("{}", s1==s2);
	println!("{}", ptr::eq(s1, s2));

	println!("{}", s1==s3);
	println!("{}", ptr::eq(s1, s3));

    println!("{}", s1==s4);
	println!("{}", ptr::eq(s1, s4));
}
