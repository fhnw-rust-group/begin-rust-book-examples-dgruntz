#![allow(unused_variables)]
#![allow(dead_code)]

fn sum(arr: &[i32]) -> i32 {
	let mut sum = 0;
	let mut i = 0;
	while i < arr.len() {
		sum += arr[i];
		i += 1
	}
	sum
}

fn sum1(arr: &mut [i32]) -> i32 {
	arr[0] = 0;

	let mut sum = 0;
	let mut i = 0;
	while i < arr.len() {
		sum += arr[i];
		i += 1
	}
	sum
}

fn sum2(mut arr: [i32;5]) -> i32 {
	let mut sum = 0;
	let mut i = 0;
	while i < arr.len() {
		sum += arr[i];
		i += 1
	}
	arr[0] = 0;
	sum
}



pub fn main() {
	println!("Test1");
	let mut numbers = [2,3,8,1,9];

	println!("{:?}", numbers);

	println!("sum = {}", sum(&numbers));
	println!("sum = {}", sum1(&mut numbers));
	println!("sum = {}", sum2(numbers));	// bei diesem Aufruf wird das Array kopiert

	let mut i = 0;
	while i < numbers.len() {
		println!("numbers[{}] == {}", i, numbers[i]);
		i += 1
	}

	numbers[2] = 10;
}
