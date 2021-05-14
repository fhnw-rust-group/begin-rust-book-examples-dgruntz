#![allow(unused_variables)]
#![allow(dead_code)]

pub fn maximum(numbers: &[u32]) -> u32 {
    let mut max: u32 = 0;
    let mut i = 0;
    while i < numbers.len() {
        let elem = numbers[i];
        max = if elem > max { elem } else { max };
        i += 1;
    }
    max
}

pub fn reverse(numbers: &[i32]) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut i = numbers.len();
    while i > 0 {
        i -= 1;
        res.push(numbers[i]);
    }
    res
}

pub fn fibs(count: usize) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    let mut i = 0;
    while i < count {
        if i < 2 {
            res.push(1)
        } else {
            res.push(res[i-1] + res[i-2])
        }
        i += 1
    }
    res
}

pub fn main() {
    // let numbers = [1,2,3,4,5];
    // println!("{}", maximum(&numbers));
    // println!("{}", maximum(&[]));

    assert_eq!(maximum(&[1, 2, 3]), 3);
    assert_eq!(maximum(&[1, 8, 3]), 8);
    assert_eq!(maximum(&[1, 1, 1]), 1);
    assert_eq!(maximum(&[]), 0);

    let numbers = &[4, 5, 2, 8];
    assert_eq!(reverse(numbers), &[8, 2, 5, 4]);

    assert_eq!(fibs(5), &[1, 1, 2, 3, 5]);
    assert_eq!(fibs(1), &[1]);
    assert_eq!(fibs(0), &[]);
    assert_eq!(fibs(8), &[1, 1, 2, 3, 5, 8, 13, 21]);
    println!("Success!");
}