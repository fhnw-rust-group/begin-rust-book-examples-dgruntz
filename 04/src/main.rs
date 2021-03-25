fn main() {
    let x = 2;
    let y = 3;

//    assert!(x == y);
//  thread 'main' panicked at 'assertion failed: x == y', src\main.rs:4:5

//    assert_eq!(x, y);
//  thread 'main' panicked at 'assertion failed: `(left == right)`
//    left: `2`,
//   right: `3`', src\main.rs:6:5

    println!("false && true: {}", a(false) && b(true));
    println!("true || true: {}", a(true) || b(true));

    println!("false < true: {}", false < true);
    
    assert!(can_drive("Michael", 18));
    assert!(!can_drive("Michael", 17));
    assert!(can_drive("Miriam", 16));
    assert!(!can_drive("Miriam", 15));

    assert!(can_see_movie(17, false));
    assert!(can_see_movie(21, false));
    assert!(!can_see_movie(13, false));
    assert!(can_see_movie(13, true));
    assert!(!can_see_movie(12, true));

    println!("Success!");
    
}

fn can_drive(name: &str, age: u32) -> bool {
    (name != "Michael" && age >= 16) ||
    (age >= 18)
}

fn can_see_movie(age: u32, permission: bool) -> bool {
    age >= 17 || permission && age >= 13
}

fn a(x: bool) -> bool {
    println!("a() called");
    x
}

fn b(x: bool) -> bool {
    println!("b() called");
    x
}

fn foo() {
    unimplemented!();
}