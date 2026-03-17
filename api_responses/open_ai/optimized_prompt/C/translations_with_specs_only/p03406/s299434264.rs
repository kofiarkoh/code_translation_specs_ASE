const MD: i64 = 1000000007; // Predefined constant for modulo operations

fn factorial(n: usize) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = (fact[i - 1] * i as i64) % MD; // Compute factorial modulo MD
    }
    fact
}

fn inverse_factorial(n: usize, fact: &Vec<i64>) -> Vec<i64> {
    let mut inv_fact = vec![1; n + 1];
    inv_fact[n] = mod_inverse(fact[n]); // Compute inverse of n! using modular inverse
    for i in (1..n).rev() {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1) as i64) % MD; // Compute inverse factorials
    }
    inv_fact
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(x: i64) -> i64 {
    let mut res = 1;
    let mut exp = MD - 2; // Since MD is prime, we can use Fermat's theorem
    let mut base = x % MD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % MD;
        }
        base = (base * base) % MD;
        exp /= 2;
    }
    res
}

fn combinatorial(n: usize, k: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> i64 {
    if k > n {
        return 0; // Return 0 for invalid combinations
    }
    (fact[n] * inv_fact[k] % MD * inv_fact[n - k] % MD) % MD // C(n, k) = n! / (k! * (n-k)!)
}

fn compute_combinatorial(n: usize, m: usize, values: &[usize]) -> i64 {
    let fact = factorial(n);
    let inv_fact = inverse_factorial(n, &fact);
    
    let mut dp = vec![0; n + 1]; // Dynamic programming array
    dp[0] = 1; // Base case: one way to choose 0 items

    for &value in values.iter() {
        for j in (1..=n).rev() {
            dp[j] = (dp[j] + dp[j - 1] * combinatorial(n, value, &fact, &inv_fact)) % MD; // Update dp array
        }
    }

    dp[n] // Return the computed combinatorial value
}

fn main() {
    let n: usize = 5; // Example input for n
    let m: usize = 3; // Example input for m
    let values: Vec<usize> = vec![1, 2, 3]; // Example input array

    // Preconditions check
    assert!(n <= 16);
    assert!(m <= 16);
    for &value in &values {
        assert!(value <= (1 << n) - 1);
    }

    let result = compute_combinatorial(n, m, &values);
    println!("{}", result); // Output the result
}
