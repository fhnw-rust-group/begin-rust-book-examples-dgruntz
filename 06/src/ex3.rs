pub fn print_border(h: i32, w: i32) {
    print_full_row(w);
    let mut r = 0;
    while r < h-2 {
        print_empty_row(w);
        r += 1;
    }
    print_full_row(w);
}

fn print_full_row(w: i32) {
    let mut c = 0;
    while c < w {
        print!("*");
        c += 1;
    }
    println!()
}

fn print_empty_row(w: i32) {
    print!("*");
    let mut c = 0;
    while c < w-2 {
        print!(" ");
        c += 1;
    }
    println!("*");
    
}