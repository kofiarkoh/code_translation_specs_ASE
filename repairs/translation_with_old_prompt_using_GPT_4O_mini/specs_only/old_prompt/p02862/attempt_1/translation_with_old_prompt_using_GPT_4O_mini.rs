use std::io;

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let y: i64 = iter.next().unwrap().parse().unwrap();

    // Check preconditions
    if x < 0 || y < 0 || (x + y) % 3 != 0 {
        println!("0");
        return;
    }

    // Calculate the number of unique paths
    let total_moves = x + y;
    let k = total_moves / 3;
    let a = x - k;
    let b = y - k;

    // Check if the calculated a and b are valid
    if a < 0 || b < 0 {
        println!("0");
        return;
    }

    // Initialize combinatorial values
    let (fac, finv) = COMinit(total_moves as usize);

    // Calculate the number of combinations
    let result = (fac[total_moves as usize] * finv[a as usize] % MOD) * finv[b as usize] % MOD;
    println!("{}", result);
}

// Precompute factorials and inverse factorials
fn COMinit(n: usize) -> (Vec<i64>, Vec<i64>) {
    let mut fac = vec![1; n + 1];
    let mut finv = vec![1; n + 1];
    let mut inv = vec![1; n + 1];

    for i in 2..=n {
        fac[i] = fac[i - 1] * i as i64 % MOD;
    }

    inv[n] = mod_inverse(fac[n]);
    for i in (1..n).rev() {
        inv[i] = inv[i + 1] * (i + 1) as i64 % MOD;
    }

    for i in 0..=n {
        finv[i] = inv[i] * fac[i] % MOD;
    }

    (fac, finv)
}

// Function to compute modular inverse using Fermat's Little Theorem
fn mod_inverse(a: i64) -> i64 {
    let mut m = MOD - 2;
    let mut result = 1;
    let mut base = a;

    while m > 0 {
        if m % 2 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        m /= 2;
    }
    result
}
