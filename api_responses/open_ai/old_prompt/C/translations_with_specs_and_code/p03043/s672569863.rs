use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: i32 = iter.next().unwrap().parse().expect("Invalid input for K");

    // Ensure N is positive as per the specification
    if n == 0 {
        eprintln!("N must be a positive integer.");
        return;
    }

    let mut multi = 1;
    let mut denomi = vec![0; n];
    let mut nume = 0.0;

    for i in (0..n).rev() {
        denomi[i] = 0;
        while k > multi * (i + 1) as i32 {
            multi *= 2;
        }
        denomi[i] += multi * n as i32;
    }

    for j in 0..n {
        nume += denomi[0] as f64 / denomi[j] as f64;
    }

    // Output the result formatted to 12 decimal places
    println!("{:.12}", nume / denomi[0] as f64);
}
