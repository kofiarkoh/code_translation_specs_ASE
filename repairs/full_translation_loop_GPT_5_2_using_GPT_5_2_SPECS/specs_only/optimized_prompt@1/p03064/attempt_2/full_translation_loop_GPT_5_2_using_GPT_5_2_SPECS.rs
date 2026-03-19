use std::io::{self, Read};

const MOD: i64 = 998_244_353;
const MAX_SUM: usize = 90310;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_pow(mut a: i64, mut e: usize) -> i64 {
    a = mod_norm(a);
    let mut r: i64 = 1;
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    // Narrow interpretation consistent with the failing sample:
    // the first integer is the count n, followed by exactly n values.
    // If fewer than n values exist, we process what is available (EOF stops normally).
    let n_decl: usize = match it.next() {
        Some(s) => s.parse::<usize>().unwrap_or(0),
        None => return,
    };

    let mut d = vec![0i64; MAX_SUM];
    let mut e = vec![0i64; MAX_SUM];
    d[0] = 1;
    e[0] = 1;

    let mut n: usize = 0;
    let mut sum_s: usize = 0;

    for _ in 0..n_decl {
        let x: usize = match it.next() {
            Some(s) => match s.parse::<usize>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        let old_sum = sum_s;
        let new_sum = old_sum + x;

        for k in (0..=old_sum).rev() {
            let nk = k + x;
            let dk = d[k];
            let ek = e[k];
            d[nk] = (d[nk] + (2 * dk) % MOD) % MOD;
            e[nk] = (e[nk] + ek) % MOD;
        }

        sum_s = new_sum;
        n += 1;
    }

    let pow3n = mod_pow(3, n);

    let half = sum_s / 2;
    let mut sum_d: i64 = 0;
    for k in 0..=half {
        sum_d += d[k];
        if sum_d >= MOD {
            sum_d -= MOD;
        }
    }

    let mut ans = pow3n;
    ans = mod_norm(ans - (3 * sum_d) % MOD);

    if (sum_s & 1) == 0 {
        ans = (ans + (3 * e[half]) % MOD) % MOD;
    }

    println!("{}", mod_norm(ans));
}
