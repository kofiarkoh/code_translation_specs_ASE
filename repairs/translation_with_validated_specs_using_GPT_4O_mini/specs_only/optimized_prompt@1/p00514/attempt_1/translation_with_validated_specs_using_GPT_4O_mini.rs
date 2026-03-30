use std::io::{self, Write};

fn main() {
    let (n, m, r) = read_input();

    // Check preconditions
    if r < 0 {
        println!("0");
        return;
    }
    if r == 0 {
        println!("1");
        return;
    }
    if r < n * m {
        println!("0");
        return;
    }

    // Calculate the binomial coefficient (n + r - 1)C(n - 1)
    let result = binomial_coefficient(n, r);
    print_result(result);
}

fn read_input() -> (usize, usize, usize) {
    // Assuming valid input as per the specification
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (nums[0], nums[1], nums[2])
}

fn binomial_coefficient(n: usize, r: usize) -> Vec<u32> {
    let n2 = n - 1;
    let k = n + r - 1;

    // Initialize the result array to represent the value 1
    let mut result = vec![0; 100]; // Adjust size as needed for large integers
    result[0] = 1;

    // Calculate (k! / (n2! * (k - n2)!))
    for i in 0..n2 {
        multiply(&mut result, k - i);
    }
    for i in 1..=n2 {
        divide(&mut result, i);
    }

    result
}

fn multiply(result: &mut Vec<u32>, factor: usize) {
    let mut carry = 0;
    for i in 0..result.len() {
        let product = result[i] as usize * factor + carry;
        result[i] = (product % 10) as u32; // Store last digit
        carry = product / 10; // Carry over
        if carry == 0 && i == 0 {
            break; // No more carry, can stop early
        }
    }
}

fn divide(result: &mut Vec<u32>, divisor: usize) {
    let mut carry = 0;
    for i in (0..result.len()).rev() {
        let current = carry * 10 + result[i] as usize;
        result[i] = (current / divisor) as u32; // Store quotient
        carry = current % divisor; // Remainder
    }
}

fn print_result(result: Vec<u32>) {
    // Print the result, skipping leading zeros
    let mut leading_zero = true;
    for &digit in result.iter().rev() {
        if digit != 0 {
            leading_zero = false;
        }
        if !leading_zero {
            print!("{}", digit);
        }
    }
    if leading_zero {
        println!("0"); // If all digits are zero
    } else {
        println!(); // New line after the number
    }
}
