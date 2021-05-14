#![allow(unused_variables)]
#![allow(dead_code)]

struct Fruit {
    apples: i32,
    bananas: i32,
}

// A unit struct
struct Unit;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, i32);

// A struct with two fields
struct Point {
    x: i32,
    y: i32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn count_fruit(f: Fruit) -> Fruit {
    println!("i've got {} apples and {} bananas", f.apples, f.bananas);
    f
}
fn price_fruit(f: Fruit) -> i32 {
    f.apples * 8 + f.bananas * 12
}

fn increase_fruit(mut f: Fruit) -> Fruit {
    // Fruit{ apples: f.apples + 1, bananas: f.bananas + 1}
    f.apples += 1;
    f
}

pub fn main() {
    let mut f: Fruit = Fruit{ bananas: 2, apples: 1, };
    f.apples = 1;

    let f = count_fruit(f);
    let price = price_fruit(f);
    println!("I can make {} cents", price);

    // let f: Fruit = Fruit{ bananas: 2, apples: 1, };
    // let f2 = f;
    // let x = f.bananas;

    struct F{};
    struct G();
    struct H;
    let f = F{};
    let (g1, g2, g3) = (G, G(), G{});
    let (h1, h2) = (H, H{});
    
    let p12 = Pair(1,2);
    let p = Point{x: 1, y: 2};
   // let q = p;
    // p.0 = 3;
    // f.apples = 4;

    let Pair(x1, y1) = p12;
    println!("x1 = {}, y1 = {}", x1, y1);

    let Point{x: x2, y: y} = p;
    println!("xp = {}, yp = {}", x2, y);

    let u = Unit{};

    println!("Hello, world!");
//    println!("apples: {}, bananas: {}", f.apples, f.bananas);
    println!("Pair({}, {})", p12.0, p12.1);
    println!("u = {:#?}", p12);
}
