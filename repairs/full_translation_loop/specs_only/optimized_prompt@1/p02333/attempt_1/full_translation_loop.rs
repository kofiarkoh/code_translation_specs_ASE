use std::io;

const MOD: i64 = 1_000_000_007;

fn factorial(n: usize) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 2..=n {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }
    fact
}

fn mod_inverse(a: i64) -> i64 {
    // Using Fermat's Little Theorem for modular inverse
    let mut m = MOD - 2;
    let mut result = 1;
    let mut base = a;

    while m > 0 {
        if m % 2 == 1 {
            result = (result * base) % MOD;
        }
        base = (base * base) % MOD;
        m /= 2;
    }
    result
}

fn inverse_factorial(n: usize, fact: &Vec<i64>) -> Vec<i64> {
    let mut inv_fact = vec![1; n + 1];
    for i in 1..=n {
        inv_fact[i] = mod_inverse(fact[i]);
    }
    inv_fact
}

fn combinations(n: usize, k: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> i64 {
    if k > n {
        return 0;
    }
    (fact[n] * inv_fact[k] % MOD * inv_fact[n - k] % MOD) % MOD
}

fn compute_expression(n: i64, m: i64) -> i64 {
    if n < 0 || m < 0 {
        return 0; // Handle negative inputs as specified
    }

    let n = n as usize;
    let m = m as usize;

    let fact = factorial(m);
    let inv_fact = inverse_factorial(m, &fact);

    let mut result = 0;
    for k in 0..=m {
        let comb = combinations(m, k, &fact, &inv_fact);
        result = (result + comb * mod_exp(k as i64, n)) % MOD;
    }
    result
}

fn mod_exp(base: i64, exp: i64) -> i64 {
    let mut result = 1;
    let mut b = base;
    let mut e = exp;

    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % MOD;
        }
        b = (b * b) % MOD;
        e /= 2;
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if inputs.len() != 2 {
        println!("0"); // Handle invalid input format
        return;
    }

    let n = inputs[0];
    let m = inputs[1];

    let result = compute_expression(n, m);
    println!("{}", result);
}
