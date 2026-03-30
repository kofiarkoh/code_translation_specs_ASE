const MAX_FACTORIAL: usize = 2000;
const MODULO: i64 = 1_000_000_007; // Example modulo constant, adjust as needed

fn modular_exponentiation(base: i64, exp: i64, modulus: i64) -> i64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1; // Divide exp by 2
        base = (base * base) % modulus;
    }
    result
}

fn factorials_and_inverses(max: usize) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![1; max + 1];
    let mut inv_fact = vec![1; max + 1];

    for i in 2..=max {
        fact[i] = (fact[i - 1] * i as i64) % MODULO;
    }

    inv_fact[max] = modular_exponentiation(fact[max], MODULO - 2, MODULO); // Fermat's little theorem for inverse
    for i in (1..max).rev() {
        inv_fact[i] = (inv_fact[i + 1] * (i + 1) as i64) % MODULO;
    }

    (fact, inv_fact)
}

fn combination(n: i64, r: i64, fact: &[i64], inv_fact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    (fact[n as usize] * inv_fact[r as usize] % MODULO * inv_fact[(n - r) as usize] % MODULO) % MODULO
}

fn compute_expression(n: i64, m: i64) -> i64 {
    if n < 0 || m < 0 || m > MAX_FACTORIAL as i64 {
        return 0; // Invalid input handling
    }

    let (fact, inv_fact) = factorials_and_inverses(MAX_FACTORIAL);
    let mut result = 0;

    for i in 0..=m {
        let term = combination(m, i, &fact, &inv_fact);
        let exponent = modular_exponentiation(2, n, MODULO); // Base is 2 as per the specification
        let value = (term * exponent) % MODULO;

        // Adjust result based on the parity of (m - i)
        if (m - i) % 2 == 0 {
            result = (result + value) % MODULO; // Even: add
        } else {
            result = (result - value + MODULO) % MODULO; // Odd: subtract, ensure non-negative
        }
    }

    result
}

fn main() {
    // Read input values for n and m
    let n: i64 = 3; // Example exponent
    let m: i64 = 2; // Example upper limit for combinations
    let result = compute_expression(n, m);
    println!("{}", result); // Output the result
}
