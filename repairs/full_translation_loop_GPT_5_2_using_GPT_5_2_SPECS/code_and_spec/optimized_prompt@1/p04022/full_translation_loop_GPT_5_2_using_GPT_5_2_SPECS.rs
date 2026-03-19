use std::cmp::max;
use std::io::{self, Read};

const LLSQRT_HI: i64 = 1_000_000_000;
const PRIME_SIEVE_LIMIT: usize = 2222;
const RESIDUAL_LIMIT: i64 = 100_000;
const COMP_LIMIT: i64 = 10_000_000_000;
const LLS_INF: i64 = 4_154_118_101_919_364_364;

fn make_primes() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    // Matches C: sieve primes < 2222, and precompute p^2, p^3.
    let mut is_comp = vec![false; PRIME_SIEVE_LIMIT];
    let mut pl = Vec::new();
    let mut dpl = Vec::new();
    let mut trpl = Vec::new();

    for i in 2..PRIME_SIEVE_LIMIT {
        if is_comp[i] {
            continue;
        }
        let p = i as i64;
        pl.push(p);
        dpl.push(p * p);
        trpl.push(p * p * p);
        let mut j = i * 2;
        while j < PRIME_SIEVE_LIMIT {
            is_comp[j] = true;
            j += i;
        }
    }
    (pl, dpl, trpl)
}

fn llsqr(x: i64) -> i64 {
    // Smallest integer st such that st*st >= x (as in C).
    let mut st: i64 = 1;
    let mut fi: i64 = LLSQRT_HI;
    while st <= fi {
        let te = (st + fi) / 2;
        // Spec notes overflow is undefined; C uses te*te in i64.
        if te * te < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st
}

#[derive(Clone, Copy)]
struct Sd {
    val: i64,
    node: i64,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let (pl, dpl, trpl) = make_primes();
    let ls = pl.len();

    let mut reduced: Vec<i64> = Vec::with_capacity(n);
    let mut r: i64 = 0;

    for _ in 0..n {
        let t0: i64 = it.next().unwrap().parse().unwrap();
        let mut t = t0;
        let mut w = t0;

        // Remove all occurrences of small primes from t (for residual test),
        // and remove all prime-cube factors from w (cube-free reduction).
        for j in 0..ls {
            let p = pl[j];
            while t % p == 0 {
                t /= p;
            }
            let p3 = trpl[j];
            while w % p3 == 0 {
                w /= p3;
            }
        }

        // Unprocessable-large-residual rule.
        if t > RESIDUAL_LIMIT {
            let l = llsqr(t);
            if l * l != t {
                r += 1;
                continue;
            }
        }

        reduced.push(w);
    }

    if reduced.is_empty() {
        print!("{r}\n");
        return;
    }

    reduced.sort_unstable();

    // Build grouped (value, count) array like C's dat[].
    let mut dat: Vec<Sd> = Vec::new();
    let mut cur_val = reduced[0];
    let mut cur_cnt: i64 = 1;
    for &x in reduced.iter().skip(1) {
        if x != cur_val {
            dat.push(Sd {
                val: cur_val,
                node: cur_cnt,
            });
            cur_val = x;
            cur_cnt = 1;
        } else {
            cur_cnt += 1;
        }
    }
    dat.push(Sd {
        val: cur_val,
        node: cur_cnt,
    });
    dat.push(Sd {
        val: LLS_INF,
        node: 0,
    }); // sentinel, as in C

    let t = dat.len() - 1; // number of real groups
    let mut used = vec![false; dat.len()];

    for i in 0..t {
        if used[i] {
            continue;
        }
        used[i] = true;

        if dat[i].val == 1 {
            // SPEC says: add exactly 1 for v=1 regardless of multiplicity.
            // This matches the C code behavior.
            r += 1;
            continue;
        }

        // Compute complementary value w for dat[i].val.
        let mut w: i64 = 1;
        let mut h: i64 = dat[i].val;

        for j in 0..ls {
            let p = pl[j];

            // Remove all occurrences from h (residual tracking).
            while h % p == 0 {
                h /= p;
            }

            // For the original value, decide exponent mod 3 via divisibility by p and p^2.
            if dat[i].val % p == 0 {
                if dat[i].val % dpl[j] == 0 {
                    // exponent mod 3 == 2 => need one more p
                    w *= p;
                    if w > COMP_LIMIT {
                        w = -1;
                        break;
                    }
                } else {
                    // exponent mod 3 == 1 => need p^2
                    w *= p;
                    if w > COMP_LIMIT {
                        w = -1;
                        break;
                    }
                    w *= p;
                    if w > COMP_LIMIT {
                        w = -1;
                        break;
                    }
                }
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        // Handle remaining factor h (after removing small primes).
        if h > RESIDUAL_LIMIT {
            // h is a perfect square (otherwise it would have been discarded earlier).
            let l = llsqr(h);
            w *= l;
            if w > COMP_LIMIT {
                w = -1;
            }
        } else {
            // Treat h as a prime with exponent 1 => need h^2.
            w *= h;
            if w > COMP_LIMIT {
                w = -1;
            }
            if w != -1 {
                w *= h;
                if w > COMP_LIMIT {
                    w = -1;
                }
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        // Binary search for group with value w.
        let mut st: usize = 0;
        let mut fi: i64 = (t as i64) - 1;
        while (st as i64) <= fi {
            let te = ((st as i64 + fi) / 2) as usize;
            if dat[te].val < w {
                st = te + 1;
            } else {
                fi = te as i64 - 1;
            }
        }

        if dat[st].val != w {
            r += dat[i].node;
            continue;
        }

        used[st] = true;
        r += max(dat[i].node, dat[st].node);
    }

    print!("{r}\n");
}
