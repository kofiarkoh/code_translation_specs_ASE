use std::io::{self, BufRead};

fn main() {
    let mut railroad = [0; 10]; // Initialize an array of 10 integers
    let mut i = 0; // Index for the railroad array

    // Read from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        let car: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        if car == 0 { // 出ていく (leaving)
            if i > 0 { // Ensure we don't go out of bounds
                i -= 1;
                println!("{}", railroad[i]);
            }
        } else { // 入ってくる (entering)
            if i < railroad.len() { // Ensure we don't go out of bounds
                railroad[i] = car;
                i += 1;
            }
        }
    }
}
