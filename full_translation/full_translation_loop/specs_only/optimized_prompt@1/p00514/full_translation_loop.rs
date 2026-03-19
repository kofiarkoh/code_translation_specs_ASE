use std::io;

const TABAICHO_SIZE: usize = 128; // Assuming a size for the internal representation

fn main() {
    let (n, m, r) = read_input();
    let adjusted_r = r - n * m;

    if adjusted_r < 0 {
        println!("0");
    } else if adjusted_r == 0 {
        println!("1");
    } else {
        let result = binomial_coefficient(n + adjusted_r - 1, n - 1);
        println!("{}", result);
    }
}

// Function to read three non-negative integers from input
fn read_input() -> (usize, usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1], nums[2])
}

// Function to calculate the binomial coefficient (n+r-1)C(n-1)
fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        return 0; // C(n, k) is 0 if k > n
    }
    if k == 0 || k == n {
        return 1; // C(n, 0) and C(n, n) are both 1
    }

    let mut numerator = 1;
    let mut denominator = 1;

    for i in 0..k {
        numerator *= n - i; // Calculate the numerator
        denominator *= i + 1; // Calculate the denominator
    }

    numerator / denominator // Return the binomial coefficient
}
