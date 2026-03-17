use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse number");

    let mut a = vec![0; 102]; // Create a vector with 102 elements (0 to 101)
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Failed to parse number");
    }

    let mut count = 0;
    let mut i = 2; // Start from 2 as in the C code
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 1; // Skip the next element as in the C code
        }
        i += 1; // Move to the next element
    }

    // Print the count followed by a newline, ensuring flush
    print!("{}\n", count);
    io::stdout().flush().expect("Failed to flush stdout");
}
