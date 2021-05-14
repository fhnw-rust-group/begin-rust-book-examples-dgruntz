#![allow(unused_variables)]
#![allow(dead_code)]



pub fn main() {
    let mut numbers: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];
    println!("{:?}", numbers);
    println!("{:?}", &numbers[1..4]);

    numbers.push(33);
    println!("{:?}", numbers);

    let arr = [0,1,2,3,4,5];
    println!("{:?}", arr[1..4]);
    // let a:[i32;3] = arr[1..4];
}