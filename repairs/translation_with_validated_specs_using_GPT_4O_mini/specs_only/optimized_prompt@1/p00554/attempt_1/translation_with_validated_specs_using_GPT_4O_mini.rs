use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line containing n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: i32 = inputs.next().unwrap().parse().unwrap();
    let m: usize = inputs.next().unwrap().parse().unwrap();

    // Preconditions check
    assert!(m >= 1, "m must be >= 1");
    assert!(n >= 0, "n must be non-negative");

    // Initialize arrays
    let mut first_array = Vec::with_capacity(m);
    let mut second_array = Vec::with_capacity(m);

    // Read m pairs of integers
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut pair = line.split_whitespace();
        let first: i32 = pair.next().unwrap().parse().unwrap();
        let second: i32 = pair.next().unwrap().parse().unwrap();

        // Store values in arrays
        first_array.push(first);
        second_array.push(second);

        // Modify arrays based on the threshold condition
        if first >= n {
            // Set both elements to 0 if the first integer is >= n
            second_array[second_array.len() - 1] = 0;
        }
    }

    // Count how many first integers are >= n
    let count1 = first_array.iter().filter(|&&x| x >= n).count();

    // If count1 is >= m - 1, output 0
    if count1 >= m - 1 {
        println!("0");
        return;
    }

    // Sort the modified second array using a simple O(m^2) bubble sort
    for i in 0..second_array.len() {
        for j in 0..second_array.len() - 1 - i {
            if second_array[j] > second_array[j + 1] {
                second_array.swap(j, j + 1);
            }
        }
    }

    // Calculate the cumulative sum of the first m - count1 - 1 elements
    let sum_limit = m - count1 - 1;
    let cumulative_sum: i32 = second_array.iter().take(sum_limit).sum();

    // Output the cumulative sum
    println!("{}", cumulative_sum);
}
