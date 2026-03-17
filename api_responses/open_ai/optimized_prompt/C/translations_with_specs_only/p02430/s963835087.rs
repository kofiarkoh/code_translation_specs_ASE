use std::io::{self, Write};

fn main() {
    // Read input values for n and k
    let (n, k) = match read_input() {
        Ok(values) => values,
        Err(_) => {
            // Return an error code if input reading fails
            std::process::exit(1);
        }
    };

    // Check preconditions: k must not exceed n
    if k > n {
        return; // No output if k > n
    }

    // Generate and print all combinations
    generate_combinations(n, k);
}

// Function to read input values for n and k
fn read_input() -> Result<(usize, usize), io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let k = iter.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
    Ok((n, k))
}

// Function to generate and print all combinations of k indices from a set of size n
fn generate_combinations(n: usize, k: usize) {
    let total_combinations = 1 << n; // 2^n combinations

    for i in 0..total_combinations {
        // Count the number of set bits in i
        if count_set_bits(i) == k {
            // Print the binary pattern
            print_binary_pattern(i, n);
            // Print the selected indices
            print_selected_indices(i, n);
        }
    }
}

// Function to count the number of set bits in a number
fn count_set_bits(num: usize) -> usize {
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        count += n & 1; // Increment count if the least significant bit is set
        n >>= 1; // Right shift to check the next bit
    }
    count
}

// Function to print the binary pattern of a number
fn print_binary_pattern(num: usize, n: usize) {
    let pattern = format!("{:0width$b}", num, width = n);
    print!("{} ", pattern); // Print the binary pattern followed by a space
}

// Function to print the selected indices based on the binary pattern
fn print_selected_indices(num: usize, n: usize) {
    let mut indices = Vec::new();
    for j in 0..n {
        if (num & (1 << j)) != 0 {
            indices.push(j); // Collect the index if the corresponding bit is set
        }
    }
    // Print the indices
    println!("{:?}", indices);
}
