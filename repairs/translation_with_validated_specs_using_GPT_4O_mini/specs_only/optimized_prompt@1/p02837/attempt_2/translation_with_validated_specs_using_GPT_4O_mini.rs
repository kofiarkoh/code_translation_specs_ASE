fn max_set_bits(n: usize, a: &[usize], x: &[Vec<usize>], y: &[Vec<usize>]) -> usize {
    // Ensure n is within the specified bounds
    assert!(n <= 20, "n must be less than or equal to 20");
    
    let mut max_bits = 0;

    // Iterate through all possible combinations of integers from 0 to 2^n - 1
    for i in 0..(1 << n) {
        let mut valid = true;

        // Check validity against the constraints
        for j in 0..n {
            for k in 0..a[j] {
                let x_bit = (x[j][k] - 1) as usize; // Convert to 0-based index
                let y_bit = (y[j][k] - 1) as usize; // Convert to 0-based index

                // Check the validity conditions
                if (i & (1 << x_bit) != 0) && (i & (1 << y_bit) == 0) {
                    valid = false; // x[j][k] is set but y[j][k] is not
                    break;
                }
                if (i & (1 << y_bit) != 0) && (y[j][k] == 0) {
                    valid = false; // y[j][k] is 1 but should be 0
                    break;
                }
            }
            if !valid {
                break; // No need to check further if already invalid
            }
        }

        // If valid, count the number of set bits
        if valid {
            let set_bits_count = i.count_ones() as usize; // Count the number of set bits
            max_bits = max_bits.max(set_bits_count); // Update max if this combination has more set bits
        }
    }

    max_bits // Return the maximum number of set bits found
}

fn main() {
    // Example usage
    let n = 3;
    let a = vec![2, 1, 1];
    let x = vec![vec![1, 2], vec![2], vec![1]];
    let y = vec![vec![2, 0], vec![1], vec![0]];
    
    let result = max_set_bits(n, &a, &x, &y);
    println!("Maximum set bits: {}", result);
}
