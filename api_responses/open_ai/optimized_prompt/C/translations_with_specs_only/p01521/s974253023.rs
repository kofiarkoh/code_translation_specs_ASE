use std::io::{self, BufRead};

fn main() {
    // Read a single line of input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();

    // Initialize counts for 'o' and 'x'
    let mut count_o = 0;
    let mut count_x = 0;

    // Count occurrences of 'o' and 'x'
    for ch in input.chars() {
        if ch == 'o' {
            count_o += 1;
        } else if ch == 'x' {
            count_x += 1;
        }
    }

    // Output 'x' if count_x is less than count_o, otherwise output 'o'
    if count_x < count_o {
        println!("x");
    } else {
        println!("o");
    }
}
