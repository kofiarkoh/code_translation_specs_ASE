// This program computes a combinatorial value based on the input parameters n and m,
// utilizing factorials and modular arithmetic to handle large numbers.

fn factorial(n: usize) -> usize {
    let mut result: usize = 1; // Specify type for result
    for i in 1..=n {
        result = result.checked_mul(i).unwrap_or(0); // Handle overflow by returning 0
    }
    result
}

fn mod_inverse(a: usize, m: usize) -> usize {
    // Using Fermat's Little Theorem for modular inverse when m is prime
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;

    if m == 1 {
        return 0; // Inverse does not exist
    }

    let mut a = a % m;

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;
        let temp = y; // Use a temporary variable for y

        y = x - q * y;
        x = temp; // Assign temp to x
    }

    if x < 0 {
        x += m; // Ensure x is positive
    }

    x
}

fn combinatorial(n: usize, r: usize, m: usize) -> usize {
    if n < r {
        return 0; // Return 0 if n < r
    }

    let mut factorials = vec![0; 2011];
    let mut inv_factorials = vec![0; 2011];

    // Populate factorials and inverse factorials
    for i in 0..=2010 {
        factorials[i] = factorial(i) % m; // Store factorial mod m
    }

    for i in 0..=2010 {
        inv_factorials[i] = mod_inverse(factorials[i], m); // Store inverse factorial mod m
    }

    // Calculate C(n, r) = n! / (r! * (n - r)!)
    let result = (factorials[n] * inv_factorials[r] % m * inv_factorials[n - r] % m) % m;

    result // Return the result mod m
}

fn main() {
    let n: usize = 10; // Example input for n
    let r: usize = 5;  // Example input for r
    let m: usize = 1009; // Example modulus

    let result = combinatorial(n, r, m);
    println!("{}", result); // Output the result
}
