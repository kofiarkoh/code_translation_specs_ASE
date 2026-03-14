fn main() {
    use std::io::{self, Write};

    let mut a = [0; 5];
    let stdin = io::stdin();
    let mut input = String::new();

    // Read 5 integers from stdin
    for i in 0..5 {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Failed to parse integer");
    }

    // Sort the array in descending order using a simple bubble sort
    for i in 0..5 {
        for j in i..5 {
            if a[i] < a[j] {
                // Swap elements
                let t = a[i];
                a[i] = a[j];
                a[j] = t;
            }
        }
    }

    // Print the sorted array
    for i in 0..5 {
        print!("{} ", a[i]);
    }
    // Print a newline at the end to match the C output behavior
    println!();
}
