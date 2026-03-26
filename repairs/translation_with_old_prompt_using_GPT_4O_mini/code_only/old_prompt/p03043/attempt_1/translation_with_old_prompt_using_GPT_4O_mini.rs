use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: i32 = iter.next().unwrap().parse().unwrap();

    let mut multi = 1;
    let mut denomi = vec![0; n];
    let mut nume: f64 = 0.0;

    for i in (0..n).rev() {
        while k > multi * (i as i32 + 1) {
            multi *= 2;
        }
        denomi[i] += multi * (n as i32);
    }

    for j in 0..n {
        nume += denomi[0] as f64 / denomi[j] as f64;
    }

    // Print the result with 12 decimal places
    println!("{:.12}", nume / denomi[0] as f64);
}
