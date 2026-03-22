use std::cmp::max;
use std::io::{self, Read};

const LIM_PRIME: i64 = 2221;
const THRESH: i64 = 10_000_000_000;
const LLINF: i64 = 4_154_118_101_919_364_364;

#[derive(Clone, Copy)]
struct Sd {
    val: i64,
    node: i64,
}

fn make_primes() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let mut fl = vec![false; (LIM_PRIME as usize) + 1];
    let mut pl = Vec::new();
    let mut dpl = Vec::new();
    let mut trpl = Vec::new();
    for i in 2..=LIM_PRIME {
        if fl[i as usize] {
            continue;
        }
        pl.push(i);
        dpl.push(i * i);
        trpl.push(i * i * i);
        let mut j = 2 * i;
        while j <= LIM_PRIME {
            fl[j as usize] = true;
            j += i;
        }
    }
    (pl, dpl, trpl)
}

// Integer sqrt via binary search, matching the C routine's behavior.
// Returns the smallest st such that st*st >= x, with st in [1, 1e9] for x>=1.
fn llsqr(x: i64) -> i64 {
    let mut st: i64 = 1;
    let mut fi: i64 = 1_000_000_000;
    while st <= fi {
        let te = (st + fi) / 2;
        // te*te fits in i64 because te <= 1e9 => te^2 <= 1e18
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

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let (pl, dpl, trpl) = make_primes();

    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut r: i64 = 0;

    for _ in 0..n {
        let mut t: i64 = it.next().unwrap().parse().unwrap();
        let mut w: i64 = t;

        // Remove all factors of primes <= 2221 from t (completely),
        // and remove all cube factors p^3 from w.
        for j in 0..pl.len() {
            let p = pl[j];
            let p3 = trpl[j];

            while t % p == 0 {
                t /= p;
            }
            while w % p3 == 0 {
                w /= p3;
            }
        }

        // Classification failure rule (SPEC 5A): if remaining t > 100000 and not a perfect square
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

    // Compress into (value, count)
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
    dat.push(Sd { val: LLINF, node: 0 }); // sentinel, as in C

    let tlen = dat.len() - 1; // exclude sentinel from processing range
    let mut processed = vec![false; dat.len()];

    for i in 0..tlen {
        if processed[i] {
            continue;
        }
        processed[i] = true;

        if dat[i].val == 1 {
            // Special case: contributes exactly 1 regardless of multiplicity
            r += 1;
            continue;
        }

        // Construct complement w for v = dat[i].val
        let v = dat[i].val;
        let mut comp: i64 = 1;
        let mut h: i64 = v;

        for j in 0..pl.len() {
            let p = pl[j];
            let p2 = dpl[j];

            while h % p == 0 {
                h /= p;
            }

            if v % p == 0 {
                // exponent mod 3 is either 1 or 2 (since v is cube-reduced for these primes)
                if v % p2 == 0 {
                    // exponent 2 -> need p^1
                    comp = match comp.checked_mul(p) {
                        Some(x) => x,
                        None => -1,
                    };
                    if comp == -1 || comp > THRESH {
                        comp = -1;
                        break;
                    }
                } else {
                    // exponent 1 -> need p^2
                    comp = match comp.checked_mul(p) {
                        Some(x) => x,
                        None => -1,
                    };
                    if comp == -1 || comp > THRESH {
                        comp = -1;
                        break;
                    }
                    comp = match comp.checked_mul(p) {
                        Some(x) => x,
                        None => -1,
                    };
                    if comp == -1 || comp > THRESH {
                        comp = -1;
                        break;
                    }
                }
            }
        }

        if comp == -1 {
            r += dat[i].node;
            continue;
        }

        // Handle remaining factor h using the same >100000 perfect-square logic as in C.
        if h > 100_000 {
            let l = llsqr(h);
            comp = match comp.checked_mul(l) {
                Some(x) => x,
                None => -1,
            };
            if comp == -1 || comp > THRESH {
                comp = -1;
            }
        } else {
            comp = match comp.checked_mul(h) {
                Some(x) => x,
                None => -1,
            };
            if comp == -1 || comp > THRESH {
                comp = -1;
            } else {
                comp = match comp.checked_mul(h) {
                    Some(x) => x,
                    None => -1,
                };
                if comp == -1 || comp > THRESH {
                    comp = -1;
                }
            }
        }

        if comp == -1 {
            r += dat[i].node;
            continue;
        }

        // Binary search for comp in dat[0..tlen)
        let mut st: usize = 0;
        let mut fi: i64 = (tlen as i64) - 1;
        while (st as i64) <= fi {
            let te = ((st as i64 + fi) / 2) as usize;
            if dat[te].val < comp {
                st = te + 1;
            } else {
                fi = te as i64 - 1;
            }
        }

        if st >= tlen || dat[st].val != comp {
            r += dat[i].node;
            continue;
        }

        processed[st] = true;
        r += max(dat[i].node, dat[st].node);
    }

    print!("{r}\n");
}
