pub fn fun(rows: i32) {
    let mut r = 0;
    while(r < rows) {
        let mut c = 0;
        while c < r {
            print!("X");
            c+=1
        }
        println!();
        r += 1;
    }
}