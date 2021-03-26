fn main() {

    let mut a = 2;
    let mut b = 0;

    if a == 1 {
        if b == 1 {
            a = 42;
    } else {
        b = 42;
    }
    }

    println!("a = {}, b = {}", a, b);

}
