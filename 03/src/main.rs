fn double(x: i32) -> i32 {
    2*x
}

fn greet(name: &str, counter: i32) -> () {
    println!("Hello {}, you can have {} apples", name, counter);
}

fn main()  {
    println!("result of doulbe(1) = {}", double(1));

    greet("Alice", 10);
    greet("Bob", 8);
    greet("Charlie", 6);
}
