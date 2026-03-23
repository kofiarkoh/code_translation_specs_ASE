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

// Returns the smallest st such that st*st >= x, with st in [1, 1e9+1] per the C logic.
fn llsqr(x: i64) -> i64 {
    let mut st: i64 = 1;
    let mut fi: i64 = 1_000_000_000;
    while st <= fi {
        let te = (st + fi) / 2;
        // In C, te*te is within i64 range here (<= 1e18).
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let (pl, dpl, trpl) = makepl();
    let ls = pl.len();

    let n: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::new();
    a.reserve(n as usize);

    let mut r: i64 = 0;
    for _ in 0..n {
        let mut t: i64 = it.next().unwrap().parse().unwrap();
        let mut w: i64 = t;

        for j in 0..ls {
            let pj = pl[j];
            while t % pj == 0 {
                t /= pj;
            }
            let tr = trpl[j];
            while w % tr == 0 {
                w /= tr;
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

    let t_len = dat.len();
    dat.push(Sd {
        val: LLINF,
        node: 0,
    }); // sentinel like C: dat[t].val = llinf

    let mut tfl = vec![false; t_len];

    for i in 0..t_len {
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
            let pj = pl[j];
            while h % pj == 0 {
                h /= pj;
            }
            if dat[i].val % pj == 0 {
                if dat[i].val % dpl[j] == 0 {
                    w = w.saturating_mul(pj);
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                } else {
                    w = w.saturating_mul(pj);
                    if w > 10_000_000_000 {
                        w = -1;
                        break;
                    }
                    w = w.saturating_mul(pj);
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
            let l = llsqr(h);
            w = w.saturating_mul(l);
            if w > 10_000_000_000 {
                w = -1;
            }
        } else {
            w = w.saturating_mul(h);
            if w > 10_000_000_000 {
                w = -1;
            }
            w = w.saturating_mul(h);
            if w > 10_000_000_000 {
                w = -1;
            }
        }

        if w == -1 {
            r += dat[i].node;
            continue;
        }

        // Binary search lower_bound for w in dat[0..t_len)
        let mut st: i64 = 0;
        let mut fi: i64 = (t_len as i64) - 1;
        while st <= fi {
            let te = (st + fi) / 2;
            if dat[te as usize].val < w {
                st = te + 1;
            } else {
                fi = te - 1;
            }
        }
        let st_usize = st as usize;

        if dat[st_usize].val != w {
            r += dat[i].node;
            continue;
        }

        tfl[st_usize] = true;
        r += dat[i].node.max(dat[st_usize].node);
    }

    print!("{r}\n");
}
