use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Failed to parse N");

    let mut a = vec![0; 102]; // Create a vector with 102 elements (0 to 101)
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        a[i] = input.trim().parse().expect("Failed to parse element");
    }

    let mut count = 0;
    let mut i = 2; // Start from index 2
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 1; // Skip the next element
        }
        i += 1; // Move to the next element
    }

    print!("{}\n", count);
    io::stdout().flush().unwrap(); // Ensure output is flushed
}
