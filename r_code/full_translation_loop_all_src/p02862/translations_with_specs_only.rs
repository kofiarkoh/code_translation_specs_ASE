use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_000;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r: i64 = 1;
    a %= MOD;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn main() {
    // Read two integers x, y from stdin (assumed valid format per spec).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();

    // If (x + y) is not divisible by 3, output 0.
    let sum = x + y;
    if sum % 3 != 0 {
        println!("0");
        return;
    }

    // Adjusted coordinates derived from solving:
    // 2a + b = x, a + 2b = y  => a = (2x - y)/3, b = (2y - x)/3
    // Valid only if both are non-negative integers.
    let a_num = 2 * x - y;
    let b_num = 2 * y - x;

    // Since sum divisible by 3 implies a_num and b_num are divisible by 3 for integer x,y,
    // but we still keep the narrowest behavior: if division would not be integral, treat as invalid.
    if a_num % 3 != 0 || b_num % 3 != 0 {
        println!("0");
        return;
    }

    let a = a_num / 3;
    let b = b_num / 3;

    if a < 0 || b < 0 {
        println!("0");
        return;
    }

    let n = a + b;
    if n < 0 {
        println!("0");
        return;
    }
    let n_usize = n as usize;

    // Maintain factorial and inverse factorial arrays up to MAX (invariant per spec).
    // If required n exceeds MAX, the spec does not define behavior; choose narrowest:
    // output 0 rather than attempting dynamic resizing beyond the defined maximum.
    if n_usize > MAX {
        println!("0");
        return;
    }

    let mut fact = vec![1i64; MAX + 1];
    for i in 1..=MAX {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    let mut inv_fact = vec![1i64; MAX + 1];
    inv_fact[MAX] = mod_pow(fact[MAX], MOD - 2);
    for i in (1..=MAX).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    // Number of distinct paths corresponds to choosing positions of one move type among total:
    // C(n, a) (equivalently C(n, b)).
    let a_usize = a as usize;
    let ans = (((fact[n_usize] * inv_fact[a_usize]) % MOD) * inv_fact[n_usize - a_usize]) % MOD;
    println!("{}", ans);
}
