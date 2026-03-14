use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut car = vec![0; 100]; // Initialize car array with 100 zeros
    let mut out = vec![0; 100]; // Initialize out array with 100 zeros
    let mut i = 0; // Index for car
    let mut k = 0; // Index for out

    for line in handle.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap(); // Read and parse input
        if n == 0 {
            i -= 1; // Decrement i
            out[k] = car[i]; // Store the value in out
            k += 1; // Increment k
        } else {
            car[i] = n; // Store n in car
            i += 1; // Increment i
        }
    }

    for j in 0..k {
        println!("{}", out[j]); // Print each value in out
    }
}
