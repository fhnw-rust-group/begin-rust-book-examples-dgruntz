pub fn fun() {
    let _x = 3;
    let _x = 4.4;
    let _a;
    let _b;
    _a = 3;
    _b = 4.4;

    let _x: i32 = 3;
    let _y: u32;
    _y = 3;

    let _sum: i32;
    //  _sum += 1;
    //  ^^^^^^^^ use of possibly-uninitialized `_sum`

    let x;
    if true {
        x = 5;
    }
    // println!("{}", x);
    //                ^ use of possibly-uninitialized `x`

    println!("vars done")
}
