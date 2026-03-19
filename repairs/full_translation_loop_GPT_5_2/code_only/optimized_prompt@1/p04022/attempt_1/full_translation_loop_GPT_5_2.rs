use std::cmp::Ordering;
use std::io::{self, Read};

const LLINF: i64 = 4154118101919364364;

#[derive(Clone, Copy)]
struct Sd {
    val: i64,
    node: i64,
}

fn makepl() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut fl = vec![false; 2222];
    let mut pl: Vec<i64> = Vec::new();
    let mut dpl: Vec<i64> = Vec::new();
    let mut trpl: Vec<i64> = Vec::new();

    for i in 2..2222 {
        if fl[i] {
            continue;
        }
        let ii = i as i64;
        pl.push(ii);
        dpl.push(ii * ii);
        trpl.push(ii * ii * ii);
        let mut j = 2 * i;
        while j < 2222 {
            fl[j] = true;
            j += i;
        }
    }
    (pl, dpl, trpl)
}

// Smallest integer st such that st*st >= x, with st in [1, 1e9] (as in C).
fn llsqr(x: i64) -> i64 {
    let mut st: i64 = 1;
    let mut fi: i64 = 1_000_000_000;
    while st <= fi {
        let te = (st + fi) / 2;
        // Use i128 to avoid overflow; C used long long and relied on range here.
        let te2 = (te as i128) * (te as i128);
        if te2 < x as i128 {
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

    let (pl, dpl, trpl) = makepl();
    let ls = pl.len();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::new();
    a.reserve(n as usize);
    let mut r: i64 = 0;

    for _ in 0..n {
        let mut t: i64 = it.next().unwrap().parse().unwrap();
        let mut w: i64 = t;

        for j in 0..ls {
            let p = pl[j];
            let tp = trpl[j];

            while t % p == 0 {
                t /= p;
            }
            while w % tp == 0 {
                w /= tp;
            }
        }

        if t > 100_000 {
            let l = llsqr(t);
            if l * l != t {
                r += 1;
                continue;
            }
        }

        a.push(w);
    }

    if a.is_empty() {
        print!("{r}\n");
        return;
    }

    a.sort_unstable();

    // Build dat: run-length encoding of sorted a
    let mut dat: Vec<Sd> = Vec::new();
    dat.reserve(a.len() + 1);

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

    let tlen = dat.len();
    dat.push(Sd { val: LLINF, node: 0 }); // sentinel like C: dat[t].val = llinf

    let mut tfl = vec![false; tlen];

    for i in 0..tlen {
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

        for j in 0..ls {
            let p = pl[j];

            while h % p == 0 {
                h /= p;
            }

            if dat[i].val % p == 0 {
                if dat[i].val % dpl[j] == 0 {
                    // multiply by p once
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
                    // multiply by p twice
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
            let l = llsqr(h);
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

        // lower_bound on dat[0..tlen) by val
        let mut st: i64 = 0;
        let mut fi: i64 = (tlen as i64) - 1;
        while st <= fi {
            let te = (st + fi) / 2;
            if dat[te as usize].val < w {
                st = te + 1;
            } else {
                fi = te - 1;
            }
        }
        let st_us = st as usize;

        if dat[st_us].val != w {
            r += dat[i].node;
            continue;
        }

        tfl[st_us] = true;
        r += std::cmp::max(dat[i].node, dat[st_us].node);
    }

    print!("{r}\n");
}
