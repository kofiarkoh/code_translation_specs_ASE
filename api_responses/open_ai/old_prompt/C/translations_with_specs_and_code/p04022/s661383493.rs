use std::io::{self, BufRead};
use std::cmp::{max, min};

const LLINF: i64 = 4154118101919364364;

#[derive(Debug)]
struct Sd {
    val: i64,
    node: i64,
}

fn make_primes() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut fl = vec![false; 2222];
    let mut pl = Vec::new();
    let mut dpl = Vec::new();
    let mut trpl = Vec::new();

    for i in 2..2222 {
        if fl[i] {
            continue;
        }
        pl.push(i);
        dpl.push(i * i);
        trpl.push(i * i * i);
        for j in (2 * i..2222).step_by(i) {
            fl[j] = true;
        }
    }
    (pl, dpl, trpl)
}

fn ll_sqr(x: i64) -> i64 {
    let (mut st, mut fi) = (1, 1_000_000_000);
    while st <= fi {
        let te = (st + fi) / 2;
        if te * te < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    if n == 0 {
        println!("0");
        return;
    }

    let (pl, dpl, trpl) = make_primes();
    let mut a = Vec::new();
    let mut r = 0;

    for _ in 0..n {
        let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        let mut w = t;
        let mut temp_t = t;

        for &prime in &pl {
            while temp_t % prime == 0 {
                temp_t /= prime;
            }
            while w % trpl[pl.iter().position(|&x| x == prime).unwrap()] == 0 {
                w /= trpl[pl.iter().position(|&x| x == prime).unwrap()];
            }
        }

        if temp_t > 100_000 {
            let l = ll_sqr(temp_t);
            if l * l != temp_t {
                r += 1;
                continue;
            }
        }
        a.push(w);
    }

    if a.is_empty() {
        println!("{}", r);
        return;
    }

    a.sort();
    let mut dat = Vec::new();
    let mut t = 0;
    let mut w = 1;

    dat.push(Sd { val: a[0], node: 1 });

    for i in 1..a.len() {
        if a[i - 1] != a[i] {
            dat[t].node = w;
            t += 1;
            w = 1;
            dat.push(Sd { val: a[i], node: 0 });
        } else {
            w += 1;
        }
    }
    dat[t].node = w;
    t += 1;
    dat.push(Sd { val: LLINF, node: 0 });

    let mut tfl = vec![false; t];

    for i in 0..t {
        if tfl[i] {
            continue;
        }
        tfl[i] = true;
        if dat[i].val == 1 {
            r += 1;
            continue;
        }
        w = 1;
        let mut h = dat[i].val;

        for &prime in &pl {
            while h % prime == 0 {
                h /= prime;
            }
            if dat[i].val % prime == 0 {
                if dat[i].val % dpl[pl.iter().position(|&x| x == prime).unwrap()] == 0 {
                    w *= prime;
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                } else {
                    w *= prime;
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                    w *= prime;
                    if w > 10_000_000_000 {
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

        if h > 100_000 {
            let l = ll_sqr(h);
            w *= l;
            if w > 10_000_000_000 {
                w = -1;
            }
        } else {
            w *= h;
            if w > 10_000_000_000 {
                w = -1;
            }
            w *= h;
            if w > 10_000_000_000 {
                w = -1;
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        let mut st = 0;
        let mut fi = t - 1;

        while st <= fi {
            let te = (st + fi) / 2;
            if dat[te].val < w {
                st = te + 1;
            } else {
                fi = te - 1;
            }
        }

        if dat[st].val != w {
            r += dat[i].node;
            continue;
        }
        tfl[st] = true;
        r += max(dat[i].node, dat[st].node);
    }

    println!("{}", r);
}
