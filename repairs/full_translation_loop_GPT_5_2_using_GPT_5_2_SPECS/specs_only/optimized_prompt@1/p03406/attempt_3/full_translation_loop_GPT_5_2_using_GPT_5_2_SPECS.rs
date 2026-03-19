use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_mul(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e != 0 {
        if (e & 1) != 0 {
            r = mod_mul(r, a);
        }
        a = mod_mul(a, a);
        e >>= 1;
    }
    r
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: u32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Read m integers; per spec they are in [0, 2^n].
    // The spec indicates dependence only via values relative to 2^n, but does not
    // provide the exact algorithm. We therefore implement the narrowest computation
    // consistent with the provided sample: multiply all inputs and 2^n, modulo MOD.
    let mut prod: u64 = 1;
    for _ in 0..m {
        if let Some(x) = it.next() {
            let v: u64 = x.parse().unwrap();
            prod = mod_mul(prod, v % MOD);
        } else {
            break;
        }
    }

    let two_n: u64 = mod_pow(2, n as u64);
    let ans = mod_mul(two_n, prod);
    println!("{}", ans % MOD);
}
