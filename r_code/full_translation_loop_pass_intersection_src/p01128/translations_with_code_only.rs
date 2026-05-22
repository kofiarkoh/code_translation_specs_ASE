use std::cmp::Ordering;
use std::io::{self, Read};

const EPS: f64 = 1e-10;

#[derive(Clone, Copy, Debug)]
struct P {
    x: f64,
    y: f64,
    o: i32,
    l: i32,
}

// Emulate the (buggy) C comparator as closely as possible.
// Note: The C code uses `abs(A->x-B->x)<0` which is always false, so the y-tie-break
// branch is never taken. Thus it effectively sorts only by x using comparisons
// against 0, and returns 0 when x is "equal" (including NaN cases).
fn cmp_p(a: &P, b: &P) -> Ordering {
    let dx1 = a.x - b.x;
    if dx1 < 0.0 {
        return Ordering::Less;
    }
    let dx2 = b.x - a.x;
    if dx2 < 0.0 {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let c_cases: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..c_cases {
        let xa: i32 = it.next().unwrap().parse().unwrap();
        let ya: i32 = it.next().unwrap().parse().unwrap();
        let xb: i32 = it.next().unwrap().parse().unwrap();
        let yb: i32 = it.next().unwrap().parse().unwrap();
        let n: i32 = it.next().unwrap().parse().unwrap();

        let mut st: Vec<P> = Vec::with_capacity(105);

        for _ in 0..n {
            let xs: i32 = it.next().unwrap().parse().unwrap();
            let ys: i32 = it.next().unwrap().parse().unwrap();
            let xt: i32 = it.next().unwrap().parse().unwrap();
            let yt: i32 = it.next().unwrap().parse().unwrap();
            let o: i32 = it.next().unwrap().parse().unwrap();
            let l: i32 = it.next().unwrap().parse().unwrap();

            // Preserve C's signed int overflow/wraparound behavior by using wrapping ops.
            let tx = xs.wrapping_sub(xa);
            let ty = ys.wrapping_sub(ya);

            let b = (xb.wrapping_sub(xa))
                .wrapping_mul(yt.wrapping_sub(ys))
                .wrapping_sub((yb.wrapping_sub(ya)).wrapping_mul(xt.wrapping_sub(xs)));

            if b == 0 {
                continue;
            }

            // Numerators also computed with wrapping to emulate C int overflow.
            let num_r = (yt.wrapping_sub(ys))
                .wrapping_mul(tx)
                .wrapping_sub((xt.wrapping_sub(xs)).wrapping_mul(ty));
            let num_s = (yb.wrapping_sub(ya))
                .wrapping_mul(tx)
                .wrapping_sub((xb.wrapping_sub(xa)).wrapping_mul(ty));

            let r = (num_r as f64) / (b as f64);
            let s = (num_s as f64) / (b as f64);

            if r < EPS || s < EPS || (1.0 - r) < EPS || (1.0 - s) < EPS {
                continue;
            }

            // The else branch is unreachable given the continue above, but keep it
            // to preserve structure/intent.
            let ins = if r > EPS && (1.0 - r) > EPS {
                P {
                    x: (xa as f64) + r * ((xb.wrapping_sub(xa)) as f64),
                    y: (ya as f64) + r * ((yb.wrapping_sub(ya)) as f64),
                    o,
                    l,
                }
            } else {
                P {
                    x: (xs as f64) + s * ((xt.wrapping_sub(xs)) as f64),
                    y: (ys as f64) + s * ((yt.wrapping_sub(ys)) as f64),
                    o,
                    l,
                }
            };

            st.push(ins);
        }

        st.sort_by(cmp_p);

        let mut r_best: i32 = 999;
        for l0 in 0..2 {
            let mut t = l0;
            let mut bcnt: i32 = 0;
            for p in &st {
                if p.o == 1 && p.l != t {
                    bcnt += 1;
                    t ^= 1;
                }
                if p.o != 1 && p.l == t {
                    bcnt += 1;
                    t ^= 1;
                }
            }
            if bcnt < r_best {
                r_best = bcnt;
            }
        }

        out.push_str(&format!("{r_best}\n"));
    }

    print!("{out}");
}
