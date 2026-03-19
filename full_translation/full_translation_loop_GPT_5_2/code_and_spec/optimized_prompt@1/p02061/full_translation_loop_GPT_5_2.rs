use std::io::{self, Read};

static mut MIZUKINANA: usize = 0;
static mut FUKUENNMISATO: [[i64; 2]; 64] = [[0; 2]; 64];
static mut UTIYAMAYUMI: i64 = 0;
static mut YOSINAGASAYURI: i64 = 1;

/// Recursively enumerates all divisors `acc` of `UTIYAMAYUMI` (via its prime factorization),
/// and multiplies `YOSINAGASAYURI` by each divisor except `UTIYAMAYUMI` itself.
/// After each multiplication, clamps `YOSINAGASAYURI` to at most `2 * UTIYAMAYUMI`.
///
/// This mirrors the C code exactly. It uses `unsafe` only to access the global mutable state
/// that the original C program relies on.
fn minaseinori(idx: usize, mut acc: i64) {
    unsafe {
        if idx >= MIZUKINANA {
            if acc != UTIYAMAYUMI {
                // Use i128 to avoid overflow during multiplication; then clamp to 2*UTIYAMAYUMI.
                let prod = (YOSINAGASAYURI as i128) * (acc as i128);
                let cap = (UTIYAMAYUMI as i128) * 2;
                YOSINAGASAYURI = if prod > cap { cap } else { prod } as i64;
            }
        } else {
            let p = FUKUENNMISATO[idx][0];
            let e = FUKUENNMISATO[idx][1] as usize;
            for _ in 0..=e {
                minaseinori(idx + 1, acc);
                acc *= p;
            }
        }
    }
}

fn main() {
    // Read all input; on parse failure, exit with error code 1 (as per spec/C behavior).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let q: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) if v > 0 => v,
        _ => std::process::exit(1),
    };

    let mut n: Vec<usize> = Vec::with_capacity(q);
    let mut max_n: usize = 0;
    for _ in 0..q {
        let v: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        if v > max_n {
            max_n = v;
        }
        n.push(v);
    }

    // count[i] defined for i in [0..=max_n], with count[0]=count[1]=0.
    let mut count = vec![0i32; max_n.saturating_add(1)];

    for i in 2..=max_n {
        // Factorize i into (prime, exponent) pairs.
        let mut sumipe = i as i64;
        let mut factors: Vec<(i64, i64)> = Vec::new();

        let mut tomatu: i64 = 2;
        while tomatu * tomatu <= sumipe {
            if sumipe % tomatu == 0 {
                let mut c: i64 = 0;
                while sumipe % tomatu == 0 {
                    sumipe /= tomatu;
                    c += 1;
                }
                factors.push((tomatu, c));
            }
            tomatu += 1;
        }
        if sumipe > 1 {
            factors.push((sumipe, 1));
        }

        unsafe {
            MIZUKINANA = factors.len();
            // Populate the global factor table (as in C).
            for (idx, (p, e)) in factors.iter().enumerate() {
                FUKUENNMISATO[idx][0] = *p;
                FUKUENNMISATO[idx][1] = *e;
            }
            UTIYAMAYUMI = i as i64;
            YOSINAGASAYURI = 1;
        }

        minaseinori(0, 1);

        let prev = count[i - 1];
        let cond = unsafe { (YOSINAGASAYURI as i128) >= (i as i128) * 2 };
        count[i] = prev + if cond { 1 } else { 0 };
    }

    let mut out = String::new();
    for &v in &n {
        // v is non-negative by parsing as usize; if v > max_n, it can't happen by construction.
        out.push_str(&format!("{}\n", count[v]));
    }
    print!("{out}");
}
