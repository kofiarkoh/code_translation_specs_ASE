const N: usize = 1_000_000_007; // Define a constant for the modulus

fn factorial_mod(n: usize) -> Vec<usize> {
    let mut fact = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = (fact[i - 1] * i) % N; // Populate factorial values modulo N
    }
    fact
}

fn mod_inverse(a: usize) -> usize {
    // Using Fermat's Little Theorem for modular inverse
    let mut m = N as isize - 2; // N is prime, so a^(N-1) ≡ 1 (mod N)
    let mut result = 1;
    let mut base = a as isize;

    while m > 0 {
        if m % 2 == 1 {
            result = (result * base) % N as isize;
        }
        base = (base * base) % N as isize;
        m /= 2;
    }
    result as usize
}

fn combination_mod(n: usize, r: usize, fact: &Vec<usize>) -> usize {
    if n < r {
        return 0; // Return 0 if n < r as per the specification
    }
    let numerator = fact[n];
    let denominator = (fact[r] * fact[n - r]) % N;
    (numerator * mod_inverse(denominator)) % N // Calculate combination modulo N
}

fn compute_combinatorial_value(x: usize, y: usize, k: usize) -> usize {
    if x + y + 2 * k > 1_000_000 { // Check precondition for bounds
        return 0; // Return 0 if the precondition is violated
    }

    let n = x + y + 2 * k; // Calculate the maximum value for factorials
    let fact = factorial_mod(n); // Initialize the memoization table

    let mut result = 0; // Initialize the result accumulator

    for i in 0..=k {
        let xgr = combination_mod(x + 2 * (k - i), k - i, &fact); // Calculate xgr
        let ygr = combination_mod(y + 2 * i, i, &fact); // Calculate ygr
        result = (result + (xgr * ygr) % N) % N; // Accumulate the result
    }

    result // Return the final computed combinatorial value
}

fn main() {
    // Read input values for x, y, and k
    let x = 5; // Example input
    let y = 3; // Example input
    let k = 2; // Example input
    let result = compute_combinatorial_value(x, y, k);
    println!("{}", result); // Output the result
}
