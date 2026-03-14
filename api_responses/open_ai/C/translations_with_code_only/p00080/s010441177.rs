use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let q: i32 = input.trim().parse().expect("Failed to parse input");

        if q == -1 {
            break;
        }

        let mut x = q as f64 / 2.0;
        while !(f64::abs(x * x * x - q as f64) < 0.00001 * q as f64) {
            x = x - (x * x * x - q as f64) / (3.0 * x * x);
        }

        // Print the result with the same formatting as in C
        println!("{}", x);
    }
}
