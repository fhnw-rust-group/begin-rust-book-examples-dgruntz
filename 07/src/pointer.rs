#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn foo1(mut p: Point) -> Point {
    println!("{:?}", p);
    println!("{:p}", &p);
    p.x = 42;
    p
}

pub fn main() {
    let mut p = Point{x: 1, y: 2};
    println!("{:?}", p);
    println!("{:p}", &p);

    println!("foo1:");
    p = foo1(p);

    println!("{:?}", p);
    println!("{:p}", &p);

}