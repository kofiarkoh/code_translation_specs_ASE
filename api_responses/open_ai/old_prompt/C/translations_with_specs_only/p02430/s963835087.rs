use std::io::{self, Write};

fn main() {
    let (n, k) = read_input();

    // If k is greater than n, no valid combinations exist.
    if k > n {
        return;
    }

    // Generate and print all combinations of n bits with exactly k bits set to 1.
    generate_combinations(n, k);
}

fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().expect("Invalid input for n");
    let k: usize = parts.next().unwrap().parse().expect("Invalid input for k");

    (n, k)
}

fn generate_combinations(n: usize, k: usize) {
    let max = 1 << n; // 2^n
    for i in 0..max {
        if count_set_bits(i) == k {
            print_combination(i, n);
        }
    }
}

fn count_set_bits(num: usize) -> usize {
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        count += n & 1; // Check if the least significant bit is set
        n >>= 1; // Right shift to check the next bit
    }
    count
}

fn print_combination(pattern: usize, n: usize) {
    let mut indices = Vec::new();
    for j in 0..n {
        if (pattern & (1 << j)) != 0 {
            indices.push(j);
        }
    }
    
    // Print the pattern and the indices of bits set to 1
    print!("{}: ", pattern);
    print!("{}", indices.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
    println!(); // New line after each combination
}
