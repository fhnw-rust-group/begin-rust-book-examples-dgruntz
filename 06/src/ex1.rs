pub fn fact(x: u128) -> u128 {
    let mut result = 1;
    let mut c = 1;
    while c <= x {
        result *= c;
        c += 1;
    }
    result
}