use std::io::{self, Read};

const N: usize = 100_000;

fn mpl() -> (Vec<u8>, Vec<i32>, usize) {
    // Mirrors the C globals: isprime[100000], pr[100000], primenum
    let mut isprime = vec![1u8; N];
    isprime[0] = 0;
    isprime[1] = 0;

    let mut pr: Vec<i32> = Vec::with_capacity(N);
    for i in 2..N {
        if isprime[i] != 0 {
            pr.push(i as i32);
            let mut j = i + i;
            while j < N {
                isprime[j] = 0;
                j += i;
            }
        }
    }
    let primenum = pr.len();
    (isprime, pr, primenum)
}

fn main() {
    // Read all input, then parse as ints like repeated scanf("%d%d%d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let (_isprime, pr, primenum) = mpl();

    let mut out = String::new();

    loop {
        let m_opt = it.next();
        if m_opt.is_none() {
            break;
        }
        let a_opt = it.next();
        let b_opt = it.next();
        if a_opt.is_none() || b_opt.is_none() {
            break; // incomplete triple: scanf would fail
        }

        let m: i32 = m_opt.unwrap().parse().unwrap();
        let a: i32 = a_opt.unwrap().parse().unwrap();
        let b: i32 = b_opt.unwrap().parse().unwrap();

        if (m | a | b) == 0 {
            break;
        }

        let mut p: i32 = 0;
        let mut q: i32 = 0;

        for i in 0..primenum {
            let mut l: i32 = i as i32;
            let mut r: i32 = (primenum as i32) - 1;

            while l <= r {
                let j = (l + r) / 2;
                let pi = pr[i] as i64;
                let pj = pr[j as usize] as i64;

                // Use i64 to match C's (long long) casts and avoid overflow.
                if pi * pj <= m as i64 && (a as i64) * pj <= pi * (b as i64) {
                    l = j + 1;
                } else {
                    r = j - 1;
                }
            }

            if l > i as i32 {
                let idx = (l - 1) as usize;
                let pi = pr[i] as i64;
                let pq = pr[idx] as i64;

                if pi * pq <= m as i64
                    && (a as i64) * pq <= pi * (b as i64)
                    && pi * pq > (p as i64) * (q as i64)
                {
                    p = pr[i];
                    q = pr[idx];
                }
            }
        }

        out.push_str(&format!("{} {}\n", p, q));
    }

    print!("{out}");
}
