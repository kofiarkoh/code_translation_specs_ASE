const MAX_N: usize = 2010;

fn factorial(n: usize, m: usize) -> Vec<usize> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i) % m; // Compute factorial modulo m
    }
    fact
}

fn inverse_factorial(fact: &[usize], m: usize) -> Vec<usize> {
    let mut inv_fact = vec![1; fact.len()];
    inv_fact[fact.len() - 1] = mod_inverse(fact[fact.len() - 1], m); // Inverse of n! mod m
    for i in (1..fact.len() - 1).rev() {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1)) % m; // Compute inverse factorials
    }
    inv_fact
}

// Function to compute modular inverse using Extended Euclidean Algorithm
fn mod_inverse(a: usize, m: usize) -> usize {
    let mut m0 = m;
    let mut y = 0;
    let mut x = 1;
    let mut a = a % m;

    if m == 1 {
        return 0; // Inverse does not exist
    }

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;
        let t = y; // Make t mutable to allow reassignment

        y = x - q * y;
        x = t;
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

    let fact = factorial(n, m);
    let inv_fact = inverse_factorial(&fact, m);

    // C(n, r) = n! / (r! * (n - r)!)
    let result = (fact[n] * inv_fact[r] % m) * inv_fact[n - r] % m;
    result
}

fn compute_combinatorial_value(n: usize, m: usize) -> usize {
    if n > MAX_N || m == 0 {
        return 0; // Return 0 for out of bounds
    }
    combinatorial(n, n / 2, m) // Example usage with r = n / 2
}

fn main() {
    // Example usage of compute_combinatorial_value
    let n = 10; // Example input
    let m = 1000; // Example modulus
    let result = compute_combinatorial_value(n, m);
    println!("{}", result); // Output the result
}
