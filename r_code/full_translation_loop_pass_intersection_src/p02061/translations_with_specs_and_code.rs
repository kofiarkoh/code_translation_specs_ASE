use std::io::{self, Read};

static mut MIZUKINANA: usize = 0;
static mut FUKUENNMISATO: [[i64; 2]; 64] = [[0; 2]; 64];
static mut UTIYAMAYUMI: i64 = 0;
static mut YOSINAGASAYURI: i64 = 0;

/// Recursively enumerates all divisors `acc` of `UTIYAMAYUMI` (via its prime factorization),
/// and multiplies `YOSINAGASAYURI` by each proper divisor (i.e., divisor != n),
/// capping at 2*n as per the specification invariant.
fn minaseinori(idx: usize, mut acc: i64) {
    unsafe {
        if idx >= MIZUKINANA {
            if acc != UTIYAMAYUMI {
                // Multiply by this proper divisor, with cap at 2*n.
                // Use i128 to avoid overflow before capping.
                let prod = (YOSINAGASAYURI as i128) * (acc as i128);
                let cap = (UTIYAMAYUMI as i128) * 2;
                YOSINAGASAYURI = if prod > cap { cap as i64 } else { prod as i64 };
            }
        } else {
            let p = FUKUENNMISATO[idx][0];
            let e = FUKUENNMISATO[idx][1] as i32;
            for _ in 0..=e {
                minaseinori(idx + 1, acc);
                acc *= p;
            }
        }
    }
}

fn main() {
    // Read all input; on parse failure, exit with nonzero status and no defined output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let q: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    let mut nvals: Vec<usize> = Vec::with_capacity(q);
    let mut max_n: usize = 0;
    for _ in 0..q {
        let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
            Some(v) => v,
            None => std::process::exit(1),
        };
        if n > max_n {
            max_n = n;
        }
        nvals.push(n);
    }

    // Supported bounds per spec: max query value must not exceed 114513.
    // (If it does, behavior is undefined by spec; we do not add extra error reporting.)
    let limit = 114_513usize;
    let max_compute = max_n.min(limit);

    let mut count = vec![0i32; max_compute + 1];

    for i in 2..=max_compute {
        // Factorize i into prime powers, store in FUKUENNMISATO.
        let mut x = i as i64;
        let mut idx: usize = 0;
        let mut p: i64 = 2;
        while p * p <= x {
            if x % p == 0 {
                let mut c: i64 = 0;
                while x % p == 0 {
                    x /= p;
                    c += 1;
                }
                unsafe {
                    FUKUENNMISATO[idx][0] = p;
                    FUKUENNMISATO[idx][1] = c;
                }
                idx += 1;
            }
            p += 1;
        }
        if x > 1 {
            unsafe {
                FUKUENNMISATO[idx][0] = x;
                FUKUENNMISATO[idx][1] = 1;
            }
            idx += 1;
        }

        unsafe {
            MIZUKINANA = idx;
            UTIYAMAYUMI = i as i64;
            YOSINAGASAYURI = 1;
        }
        minaseinori(0, 1);

        let ok = unsafe { YOSINAGASAYURI >= (i as i64) * 2 };
        count[i] = count[i - 1] + if ok { 1 } else { 0 };
    }

    // Output answers.
    // For n < 2, count[n] is 0 (initialized), matching spec.
    let mut out = String::new();
    for &n in &nvals {
        // Per spec, defined only up to 114513; we mirror the C behavior of indexing directly
        // within the supported range. If n > max_compute, spec does not define behavior.
        if n <= max_compute {
            out.push_str(&format!("{}\n", count[n]));
        } else {
            // Undefined by spec; do not invent behavior. Exit nonzero with no defined output.
            std::process::exit(1);
        }
    }
    print!("{out}");
}
