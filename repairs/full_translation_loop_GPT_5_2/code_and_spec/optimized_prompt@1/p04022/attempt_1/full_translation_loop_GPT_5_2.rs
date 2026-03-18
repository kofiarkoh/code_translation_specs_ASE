use std::cmp::max;
use std::io::{self, Read};

const LLINF: i64 = 4_154_118_101_919_364_364;

#[derive(Clone, Copy)]
struct Sd {
    val: i64,
    node: i64,
}

fn make_primes_upto_2221() -> Vec<i64> {
    let mut fl = vec![false; 2222];
    let mut pl = Vec::new();
    for i in 2..2222 {
        if fl[i] {
            continue;
        }
        pl.push(i as i64);
        let mut j = i * 2;
        while j < 2222 {
            fl[j] = true;
            j += i;
        }
    }
    pl
}

// Smallest integer s such that s*s >= x, with search range [1, 1e9] like the C code.
fn llsqr_ceil(x: i64) -> i64 {
    let mut st: i64 = 1;
    let mut fi: i64 = 1_000_000_000;
    while st <= fi {
        let te = (st + fi) / 2;
        // te*te fits in i64: (1e9)^2 = 1e18 < 9e18
        if te * te < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n_opt = it.next();
    if n_opt.is_none() {
        // No integers processed => output zero (spec).
        println!("0");
        return;
    }
    let n: usize = n_opt.unwrap().parse().unwrap();

    let pl = make_primes_upto_2221();
    let mut dpl: Vec<i64> = Vec::with_capacity(pl.len());
    let mut trpl: Vec<i64> = Vec::with_capacity(pl.len());
    for &p in &pl {
        dpl.push(p * p);
        trpl.push(p * p * p);
    }

    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut r: i64 = 0;

    for _ in 0..n {
        let t_str = it.next().unwrap_or("0");
        let mut t: i64 = t_str.parse().unwrap();
        let mut w: i64 = t;

        for j in 0..pl.len() {
            let p = pl[j];
            while t % p == 0 {
                t /= p;
            }
            let p3 = trpl[j];
            while w % p3 == 0 {
                w /= p3;
            }
        }

        if t > 100_000 {
            let l = llsqr_ceil(t);
            if l * l != t {
                r += 1;
                continue;
            }
        }
        a.push(w);
    }

    if a.is_empty() {
        println!("{r}");
        return;
    }

    a.sort_unstable();

    // Run-length encode into dat
    let mut dat: Vec<Sd> = Vec::new();
    let mut cur_val = a[0];
    let mut cur_cnt: i64 = 1;
    for &v in a.iter().skip(1) {
        if v != cur_val {
            dat.push(Sd {
                val: cur_val,
                node: cur_cnt,
            });
            cur_val = v;
            cur_cnt = 1;
        } else {
            cur_cnt += 1;
        }
    }
    dat.push(Sd {
        val: cur_val,
        node: cur_cnt,
    });
    let t = dat.len();

    // Sentinel like C code (dat[t].val = llinf) to allow binary search without bounds checks.
    dat.push(Sd {
        val: LLINF,
        node: 0,
    });

    let mut tfl = vec![false; t + 1];

    for i in 0..t {
        if tfl[i] {
            continue;
        }
        tfl[i] = true;

        if dat[i].val == 1 {
            r += 1;
            continue;
        }

        let mut w: i64 = 1;
        let mut h: i64 = dat[i].val;

        for j in 0..pl.len() {
            let p = pl[j];
            while h % p == 0 {
                h /= p;
            }
            if dat[i].val % p == 0 {
                // Multiply w by p or p^2 depending on whether p^2 divides dat[i].val
                if dat[i].val % dpl[j] == 0 {
                    w = match w.checked_mul(p) {
                        Some(v) => v,
                        None => -1,
                    };
                    if w > 10_000_000_000 {
                        w = -1;
                    }
                    if w == -1 {
                        break;
                    }
                } else {
                    w = match w.checked_mul(p) {
                        Some(v) => v,
                        None => -1,
                    };
                    if w > 10_000_000_000 {
                        w = -1;
                    }
                    if w == -1 {
                        break;
                    }
                    w = match w.checked_mul(p) {
                        Some(v) => v,
                        None => -1,
                    };
                    if w > 10_000_000_000 {
                        w = -1;
                    }
                    if w == -1 {
                        break;
                    }
                }
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        if h > 100_000 {
            let l = llsqr_ceil(h);
            w = match w.checked_mul(l) {
                Some(v) => v,
                None => -1,
            };
            if w > 10_000_000_000 {
                w = -1;
            }
        } else {
            w = match w.checked_mul(h) {
                Some(v) => v,
                None => -1,
            };
            if w > 10_000_000_000 {
                w = -1;
            }
            if w != -1 {
                w = match w.checked_mul(h) {
                    Some(v) => v,
                    None => -1,
                };
                if w > 10_000_000_000 {
                    w = -1;
                }
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        // Lower_bound for w in dat[0..t] (sentinel at dat[t])
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

        tfl[st] = true;
        r += max(dat[i].node, dat[st].node);
    }

    println!("{r}");
}
