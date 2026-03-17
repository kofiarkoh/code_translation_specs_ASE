// This program calculates a specific combinatorial value based on the input parameters,
// utilizing factorials and modular arithmetic to ensure results remain within a defined range.

fn factorial(n: usize) -> u64 {
    // Calculate factorial iteratively to avoid stack overflow for large n
    let mut result = 1;
    for i in 1..=n {
        result = result.checked_mul(i as u64).expect("Overflow in factorial calculation");
    }
    result
}

fn mod_inverse(a: u64, m: u64) -> u64 {
    // Using Extended Euclidean Algorithm to find modular inverse
    let (mut m0, mut y, mut x) = (m, 0, 1);
    let mut a = a % m;

    if a == 0 {
        return 0; // Inverse doesn't exist
    }

    while a > 1 {
        let q = a / m0;
        let t = m0;

        m0 = a % m0;
        a = t;
        t = y;

        y = x - q * y;
        x = t;
    }

    if x < 0 {
        x += m; // Ensure x is positive
    }

    x
}

fn combinatorial(n: usize, r: usize, m: u64) -> u64 {
    // Return 0 for invalid combinations
    if n < r {
        return 0;
    }

    // Precompute factorials and inverse factorials up to 2010
    let max_n = 2010;
    let mut fact = vec![1; max_n + 1];
    let mut inv_fact = vec![1; max_n + 1];

    for i in 1..=max_n {
        fact[i] = fact[i - 1].checked_mul(i as u64).expect("Overflow in factorial calculation") % m;
    }

    inv_fact[max_n] = mod_inverse(fact[max_n], m);
    for i in (1..max_n).rev() {
        inv_fact[i] = inv_fact[i + 1].checked_mul((i + 1) as u64).expect("Overflow in inverse factorial calculation") % m;
    }

    // Calculate C(n, r) = n! / (r! * (n - r)!)
    let result = (fact[n] * inv_fact[r] % m) * inv_fact[n - r] % m;

    result
}

fn main() {
    // Example usage
    let n: usize = 10; // Total number of items
    let r: usize = 3;  // Number of items to choose
    let m: u64 = 1000; // Modulus for calculations

    let result = combinatorial(n, r, m);
    println!("{}", result); // Output the computed combinatorial value
}
