const TABAICHO_SIZE: usize = 100; // Fixed size for large number representation
const TABAICHO_BLOCK: u32 = 1_000_000_000; // Base for large number representation

// Represents a large integer using an array of u32
struct LargeInt {
    digits: [u32; TABAICHO_SIZE],
}

impl LargeInt {
    // Initializes a LargeInt to zero
    fn new() -> Self {
        LargeInt { digits: [0; TABAICHO_SIZE] }
    }

    // Multiplies the large integer by a scalar
    fn multiply(&mut self, scalar: u32) {
        let mut carry = 0;
        for i in 0..TABAICHO_SIZE {
            let product = self.digits[i] as u64 * scalar as u64 + carry;
            self.digits[i] = (product % TABAICHO_BLOCK as u64) as u32;
            carry = (product / TABAICHO_BLOCK as u64) as u32;
        }
    }

    // Divides the large integer by a scalar
    fn divide(&mut self, scalar: u32) {
        let mut remainder = 0;
        for i in (0..TABAICHO_SIZE).rev() {
            let current = remainder * TABAICHO_BLOCK + self.digits[i] as u64;
            self.digits[i] = (current / scalar as u64) as u32;
            remainder = (current % scalar as u64) as u32;
        }
    }

    // Computes the binomial coefficient (n+r-1)C(n-1)
    fn binomial_coefficient(n: u32, r: u32) -> Self {
        let mut result = LargeInt::new();
        if r < 0 {
            // If r < 0, output is "0"
            result.digits[0] = 0;
            return result;
        } else if r == 0 {
            // If r == 0, output is "1"
            result.digits[0] = 1;
            return result;
        }

        let n_r = n + r - 1;
        let k = n - 1;

        // Calculate (n+r-1)C(n-1) = (n_r)! / (k! * (n_r - k)!)
        let mut numerator = LargeInt::new();
        let mut denominator = LargeInt::new();

        // Calculate numerator: (n_r) * (n_r - 1) * ... * (n_r - k + 1)
        for i in 0..k {
            numerator.multiply(n_r - i);
        }

        // Calculate denominator: k!
        for i in 1..=k {
            denominator.multiply(i);
        }

        // Divide numerator by denominator to get the result
        numerator.divide(denominator.digits[0]); // Assuming denominator is small enough to fit in a single block

        result = numerator;
        result
    }
}

fn main() {
    let n: u32 = 5; // Example input
    let m: u32 = 3; // Example input
    let r: u32 = 2; // Example input

    let adjusted_r = r + m; // Adjust r based on m
    let result = LargeInt::binomial_coefficient(n, adjusted_r);

    // Print the result
    println!("{}", result.digits[0]); // Output the computed binomial coefficient
}
