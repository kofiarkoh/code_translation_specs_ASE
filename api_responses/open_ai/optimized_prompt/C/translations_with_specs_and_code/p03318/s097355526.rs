use std::io::{self, Write};

fn digit_sum(x: i64) -> i64 {
    let mut r = 0;
    let mut num = x;
    while num > 0 {
        r += num % 10;
        num /= 10;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Please enter a non-negative integer");

    // Ensure the input is non-negative as per the specification
    if n < 0 {
        eprintln!("Input must be a non-negative integer.");
        return;
    }

    let mut x = 1;

    for _ in 0..n {
        println!("{}", x);
        let mut r = x + 1;
        let mut m = (x as f64) / (digit_sum(x) as f64);

        for j in 0..15 {
            let s = 10_i64.pow(j);
            let y = s * (x / s + 1) - 1;
            let o = (y as f64) / (digit_sum(y) as f64);

            if o < m {
                m = o;
                r = y;
            }
        }
        x = r;
    }
}
