use std::io;

const TABAICHO_SIZE: usize = 100; // Example size for large number representation
const TABAICHO_BLOCK: u64 = 1_000_000_000; // Base for large number representation

// A struct to represent a large number using an array of u64
struct LargeNumber {
    digits: [u64; TABAICHO_SIZE],
}

impl LargeNumber {
    fn new() -> Self {
        LargeNumber {
            digits: [0; TABAICHO_SIZE],
        }
    }

    // Function to multiply a large number by an integer
    fn multiply(&mut self, factor: u64) {
        let mut carry = 0;
        for i in 0..TABAICHO_SIZE {
            let product = self.digits[i] * factor + carry;
            self.digits[i] = product % TABAICHO_BLOCK;
            carry = product / TABAICHO_BLOCK;
        }
    }

    // Function to compute the binomial coefficient (n+r-1)C(n-1)
    fn binomial_coefficient(n: u64, r: u64) -> Self {
        let mut result = LargeNumber::new();
        result.digits[0] = 1; // Initialize result to 1

        let mut m = n + r - 1;

        for i in 0..r {
            result.multiply(m);
            result.multiply(1); // This is a no-op, just to keep the structure
            m -= 1;
            result.multiply(i + 1); // Divide by (i + 1) to adjust for binomial coefficient
        }

        result
    }

    // Function to divide the large number by an integer
    fn divide(&mut self, divisor: u64) {
        let mut remainder = 0;
        for i in (0..TABAICHO_SIZE).rev() {
            let current = remainder * TABAICHO_BLOCK + self.digits[i];
            self.digits[i] = current / divisor;
            remainder = current % divisor;
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    let n = inputs[0];
    let m = inputs[1];
    let r = inputs[2];

    let adjusted_r = if r < m { 0 } else { r - m };

    if adjusted_r < 0 {
        println!("0");
    } else if adjusted_r == 0 {
        println!("1");
    } else {
        let mut result = LargeNumber::binomial_coefficient(n, adjusted_r);
        result.divide(adjusted_r); // Adjust for the division by r in the binomial coefficient

        // Print the result (assuming the first digit is the most significant)
        let mut is_leading_zero = true;
        for &digit in result.digits.iter().rev() {
            if digit != 0 {
                if is_leading_zero {
                    is_leading_zero = false;
                }
                if !is_leading_zero {
                    print!("{}", digit);
                }
            }
        }
        if is_leading_zero {
            println!("0"); // In case the result is zero
        } else {
            println!(); // New line after the output
        }
    }
}
