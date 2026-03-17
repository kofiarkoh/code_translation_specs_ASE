use std::io::{self, BufRead};
use std::process;

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the first five integers: X, Y, A, B, C
    let mut numbers: Vec<i32> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.len() < 5 {
        // Terminate with a non-zero exit code if not enough integers
        process::exit(1);
    }

    let (x, y, a, b, c) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);

    // Ensure A, B, and C are non-negative
    if a < 0 || b < 0 || c < 0 {
        process::exit(1);
    }

    // Read the first array of A integers
    let mut first_array: Vec<i32> = stdin.lock().lines()
        .take(a as usize)
        .filter_map(|line| line.unwrap().parse().ok())
        .collect();

    // Read the second array of B integers
    let mut second_array: Vec<i32> = stdin.lock().lines()
        .take(b as usize)
        .filter_map(|line| line.unwrap().parse().ok())
        .collect();

    // Read the third array of C integers (not used in output)
    let _third_array: Vec<i32> = stdin.lock().lines()
        .take(c as usize)
        .filter_map(|line| line.unwrap().parse().ok())
        .collect();

    // Sort both arrays in descending order
    first_array.sort_by(|a, b| b.cmp(a));
    second_array.sort_by(|a, b| b.cmp(a));

    // Ensure the sizes of the arrays are at least X and Y
    if first_array.len() < x as usize || second_array.len() < y as usize {
        process::exit(1);
    }

    // Compute the sum of the first X elements of the first array
    // and the first Y elements of the second array
    let sum_first_x: i32 = first_array.iter().take(x as usize).sum();
    let sum_second_y: i32 = second_array.iter().take(y as usize).sum();

    // Output the result
    println!("{}", sum_first_x + sum_second_y);
}
