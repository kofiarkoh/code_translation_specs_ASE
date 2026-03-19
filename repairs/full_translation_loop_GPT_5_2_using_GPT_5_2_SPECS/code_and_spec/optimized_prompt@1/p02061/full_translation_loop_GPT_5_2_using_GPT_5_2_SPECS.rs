use std::io::{self, Read};

static mut MIZUKINANA: usize = 0;
static mut FUKUENNMISATO: [[i64; 2]; 64] = [[0; 2]; 64];
static mut UTIYAMAYUMI: i64 = 0;
static mut YOSINAGASAYURI: i64 = 0;

/// Recursively enumerates all divisors of `UTIYAMAYUMI` via its prime factorization
/// stored in `FUKUENNMISATO[0..MIZUKINANA)`, and multiplies into `YOSINAGASAYURI`
/// all divisors except the number itself, with saturation/capping at `2*k`.
///
/// Note: This mirrors the C code's global-state recursion. We keep it `unsafe`-free
/// by passing state explicitly in Rust, but the logic is identical.
fn minaseinori(idx: usize, acc: i64, mizukinana: usize, factors: &[[i64; 2]], k: i64, prod: &mut i64) {
    if idx >= mizukinana {
        if acc != k {
            // Multiply and cap at 2*k (spec invariant: values larger than 2k treated as 2k).
            // Use i128 to avoid intermediate overflow; spec assumes this is safe under capping.
            let cap = (k as i128) * 2;
            let mut v = (*prod as i128) * (acc as i128);
            if v > cap {
                v = cap;
            }
            *prod = v as i64;
        }
    } else {
        let p = factors[idx][0];
        let e = factors[idx][1] as usize;
        let mut cur_acc = acc;
        for _ in 0..=e {
            minaseinori(idx + 1, cur_acc, mizukinana, factors, k, prod);
            cur_acc = cur_acc * p;
        }
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let q: usize = match it.next() {
        Some(s) => match s.parse() {
            Ok(v) => v,
            Err(_) => std::process::exit(1),
        },
        None => std::process::exit(1),
    };

    let mut nvals: Vec<usize> = Vec::with_capacity(q);
    let mut max_n: usize = 0;
    for _ in 0..q {
        let v: usize = match it.next() {
            Some(s) => match s.parse() {
                Ok(v) => v,
                Err(_) => std::process::exit(1),
            },
            None => std::process::exit(1),
        };
        if v > max_n {
            max_n = v;
        }
        nvals.push(v);
    }

    // Precompute cumulative counts up to max_n
    let mut count = vec![0i32; max_n.saturating_add(1)];
    for i in 2..=max_n {
        // Factorize i
        let mut x = i as i64;
        let mut factors: [[i64; 2]; 64] = [[0; 2]; 64];
        let mut nf: usize = 0;

        let mut p: i64 = 2;
        while p * p <= x {
            if x % p == 0 {
                let mut c: i64 = 0;
                while x % p == 0 {
                    x /= p;
                    c += 1;
                }
                factors[nf][0] = p;
                factors[nf][1] = c;
                nf += 1;
            }
            p += 1;
        }
        if x > 1 {
            factors[nf][0] = x;
            factors[nf][1] = 1;
            nf += 1;
        }

        let k = i as i64;
        let mut prod: i64 = 1;
        minaseinori(0, 1, nf, &factors, k, &mut prod);

        let prev = count[i - 1];
        let add = if prod >= k * 2 { 1 } else { 0 };
        count[i] = prev + add;
    }

    // Answer queries
    let mut out = String::new();
    for &n in &nvals {
        // For n < 2, count[n] is 0 by initialization (and spec).
        let ans = if n <= max_n { count[n] } else { 0 };
        out.push_str(&format!("{ans}\n"));
    }
    print!("{out}");
}
