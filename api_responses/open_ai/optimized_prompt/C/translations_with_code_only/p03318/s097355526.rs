use std::io::{self, Write};

fn s(x: i64) -> i32 {
    let mut r = 0;
    let mut x = x;
    while x > 0 {
        r += (x % 10) as i32;
        x /= 10;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Invalid input");

    let mut x = 1;
    for _ in 0..n {
        println!("{}", x);
        let mut r = x + 1;
        let mut m = x as f64 / s(x) as f64;

        let mut s = 10;
        for j in 0..15 {
            let y = s * (x / s + 1) - 1;
            let o = y as f64 / s(y) as f64;
            if o < m {
                m = o;
                r = y;
            }
            s *= 10;
        }
        x = r;
    }
}
