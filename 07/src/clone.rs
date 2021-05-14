#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

pub struct Point2 {
    x: i32,
    y: i32,
}


pub fn main() {
    let p1 = Point{x: 1, y: 2};
    let mut p2 = p1.clone();
    p2.x = 2;
    println!("{:?}", p1);
    println!("{:?}", p2);

    let mut p1 = p1;
    let p2 = p1;           // ist ein copy wenn Copy implementiert ist.
    p1.x = 3;

//    let p2: Point2 = p1;
    let x = 1;
    let y = 2;
    let p1 = Point{x : x, y : y};
    let p1 = Point{x, y};
    let p1 = Point{x : x, y};
    println!("{:?}", p1);

    let p1 = Point{x:1, y:2};
    let p2 = Point{x:3, .. p1};
    println!("{:?}", p1);
    println!("{:?}", p2);

    struct Weekday(i32);
    let mo = Weekday(0);
}
