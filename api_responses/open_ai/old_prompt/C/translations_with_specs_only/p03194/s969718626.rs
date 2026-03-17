fn main() {
    // Read input values for n and p
    let n: i64 = read_input(); // Assume a function to read input
    let p: i64 = read_input(); // Assume a function to read input

    // Preconditions check
    if n <= 0 {
        panic!("n must be greater than 0");
    }
    if p < 0 {
        panic!("p must be non-negative");
    }

    // Initialize the product variable
    let mut ans: i64 = 1;

    // Handle edge cases for p = 0 or p = 1
    if p == 0 || p == 1 {
        println!("{}", ans);
        return;
    }

    // Factorization loop
    let mut factor = 2;
    let mut temp_p = p;

    while factor * factor <= temp_p {
        let mut count = 0;

        // Count the multiplicity of the current factor
        while temp_p % factor == 0 {
            temp_p /= factor;
            count += 1;
        }

        // If the count of this factor is at least n, multiply it to ans
        if count >= n {
            ans *= factor.pow(count as u32);
        }

        // Move to the next potential factor
        factor += 1;
    }

    // If there's any prime factor greater than sqrt(p), it can only be temp_p itself
    if temp_p > 1 && 1 >= n {
        ans *= temp_p; // temp_p is prime and appears at least once
    }

    // Output the result
    println!("{}", ans);
}

// Placeholder function for reading input
fn read_input() -> i64 {
    // Implementation for reading input goes here
    // For now, we can return a dummy value
    1 // Replace with actual input reading logic
}
