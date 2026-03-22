use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 1_000_000; // Fixed supported maximum size for precomputation.

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
    // Read two integers (no explicit malformed-input handling per spec).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();

    // Reachability checks derived from the move constraints:
    // Allowed moves are (1,2) and (2,1), each increases x+y by 3.
    let sum = x + y;
    if sum % 3 != 0 {
        println!("0");
        return;
    }
    let n = sum / 3; // total number of moves

    // Let a = # of (1,2) moves, b = # of (2,1) moves.
    // Then:
    //   x = a + 2b
    //   y = 2a + b
    // Solve:
    //   a = (2y - x)/3
    //   b = (2x - y)/3
    let a_num = 2 * y - x;
    let b_num = 2 * x - y;

    if a_num < 0 || b_num < 0 || a_num % 3 != 0 || b_num % 3 != 0 {
        println!("0");
        return;
    }

    let a = a_num / 3;
    let b = b_num / 3;

    if a + b != n {
        // Narrowest consistent interpretation: unreachable if counts don't match total moves.
        println!("0");
        return;
    }

    // Precompute factorials and inverse factorials up to MAX_N once per execution.
    // Spec says behavior is undefined if required size exceeds fixed supported maximum.
    let n_usize = n as usize;
    if n_usize > MAX_N {
        // Avoid out-of-bounds; spec allows undefined behavior here, but we keep it safe.
        // This is the narrowest safe behavior: output may be incorrect if this happens.
        println!("0");
        return;
    }

    let mut fact = vec![0i64; MAX_N + 1];
    let mut inv_fact = vec![0i64; MAX_N + 1];

    fact[0] = 1;
    for i in 1..=MAX_N {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }
    inv_fact[MAX_N] = mod_pow(fact[MAX_N], MOD - 2);
    for i in (1..=MAX_N).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    // Result is C(n, a) (equivalently C(n, b)) modulo MOD.
    let a_usize = a as usize;
    let res = (((fact[n_usize] * inv_fact[a_usize]) % MOD) * inv_fact[n_usize - a_usize]) % MOD;
    println!("{}", res);
}
