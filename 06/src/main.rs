mod blocks;
mod vars;
mod exp64;
mod ex1;
mod ex2;
mod ex3;

fn main() {
    blocks::fun();
    vars::fun();
    exp64::main();

    println!("fact(10) = {}", ex1::fact(10));

    ex2::fun(10);

    ex3::print_border(6, 5);

    let mut x = 0;
    let res: i32 = loop {
        x = x + 1;
        if x == 9 {
            break x;
        }
    };
    println!("{}", res);
}

