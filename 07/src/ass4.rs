#![allow(unused_variables)]
#![allow(dead_code)]

struct Point {  // capitalize point => Point
    x: i32,
    y: i32,
}
fn print_point(point: Point) {
    println!("x == {}, y == {}", point.x, point.y);
}
pub fn main() {
    print_point(Point { x: 3, y: -6 });

    let heap_vec: Vec<i8> = Vec::new();
    let heap_string: String = String::from("Howdy");
    let heap_i8: Box<i8> = Box::new(30);

}


