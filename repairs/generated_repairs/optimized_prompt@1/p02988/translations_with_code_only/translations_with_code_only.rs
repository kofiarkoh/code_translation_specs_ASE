use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");

    // Initialize the array with a size of n
    let mut num = vec![0; n];

    // Read the next n numbers into the array
    for i in 0..n {
        num[i] = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse number");
    }

    let mut count = 0;

    // Iterate through the array to count the conditions
    for i in 0..(n - 2) {
        if num[i] < num[i + 1] {
            if num[i + 1] < num[i + 2] {
                count += 1;
            }
        } else if num[i] > num[i + 1] {
            if num[i + 1] > num[i + 2] {
                count += 1;
            }
        }
    }

    // Print the result
    println!("{}", count);
}
