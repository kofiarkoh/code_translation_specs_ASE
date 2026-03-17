fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input");

    // Preconditions: The number of integers must be positive
    assert!(n > 0, "The number of integers must be positive");

    // Read the integers into a vector
    let mut numbers: Vec<i64> = lines
        .take(n)
        .map(|line| line.unwrap().parse().expect("Invalid input"))
        .collect();

    // Adjust numbers based on their indices
    for (i, num) in numbers.iter_mut().enumerate() {
        *num += i as i64; // Adjust each number by its index
    }

    // Sort the adjusted numbers
    numbers.sort();

    // Compute the minimum value (first element after sorting)
    let min_value = numbers[0];

    // Output the computed minimum value
    println!("{}", min_value);
}
