fn sum(low: i32, high: i32) -> i32 {
    let mut sum = 0;
    let mut c = low;
    while c <= high {
        sum+=c;
        c+=1;
    }
    sum
}

pub fn main() {
    assert_eq!(3, sum(1, 2));
    assert_eq!(2, sum(2, 2));
    assert_eq!(55, sum(1, 10));
    println!("Success!");
}
