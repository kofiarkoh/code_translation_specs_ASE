use std::cmp::Ordering;
use std::io::{self, Read};

const EPS: f64 = 1e-10;

#[derive(Clone, Copy, Default)]
struct P {
    x: f64,
    y: f64,
    o: i32,
    l: i32,
}

// Emulate the (buggy) C comparator as closely as possible.
// Notes:
// - The C code uses `abs(A->x-B->x)` where `abs` is the integer abs from stdlib,
//   but the argument is a double; this is undefined/ill-formed in C, yet many
//   compilers accept it with implicit conversion to int. The condition
//   `abs(...) < 0` is then always false for any sane abs implementation.
// - Therefore, the y-tie-break block is effectively never taken.
// - The remaining comparisons are also odd: it checks `A->x - B->x < 0` and
//   `B->x - A->x < 0`, which correspond to A.x < B.x and A.x > B.x.
// - If neither is true (including NaN cases), it returns 0.
fn cmp_c_like(a: &P, b: &P) -> Ordering {
    // The `abs(...) < 0` branch is unreachable; skip it.

    if a.x - b.x < 0.0 {
        return Ordering::Less;
    }
    if b.x - a.x < 0.0 {
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

    let mut outputs = String::new();

    for _ in 0..c_cases {
        // ST[105] zeroed in C; we only use first `c` entries.
        let mut st: Vec<P> = Vec::with_capacity(105);

        let xa: i32 = it.next().unwrap().parse().unwrap();
        let ya: i32 = it.next().unwrap().parse().unwrap();
        let xb: i32 = it.next().unwrap().parse().unwrap();
        let yb: i32 = it.next().unwrap().parse().unwrap();
        let n: i32 = it.next().unwrap().parse().unwrap();

        for _ in 0..n {
            let xs: i32 = it.next().unwrap().parse().unwrap();
            let ys: i32 = it.next().unwrap().parse().unwrap();
            let xt: i32 = it.next().unwrap().parse().unwrap();
            let yt: i32 = it.next().unwrap().parse().unwrap();
            let o: i32 = it.next().unwrap().parse().unwrap();
            let l: i32 = it.next().unwrap().parse().unwrap();

            // Preserve C's 32-bit signed overflow/wraparound behavior.
            let tx = xs.wrapping_sub(xa);
            let ty = ys.wrapping_sub(ya);

            let b = (xb.wrapping_sub(xa))
                .wrapping_mul(yt.wrapping_sub(ys))
                .wrapping_sub((yb.wrapping_sub(ya)).wrapping_mul(xt.wrapping_sub(xs)));

            if b == 0 {
                continue;
            }

            // r = ((YT-YS)*tx-(XT-XS)*ty)/b;
            // s = ((YB-YA)*tx-(XB-XA)*ty)/b;
            // Numerators computed with wrapping i32 like C.
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

            // The else branch is effectively unreachable given the continue above,
            // but we keep it to match the C structure.
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

        // qsort with the buggy comparator; Rust sort is stable, qsort isn't.
        // Stability doesn't matter here because comparator can return Equal for
        // many distinct elements; qsort's order is unspecified in that case too.
        st.sort_by(cmp_c_like);

        let c = st.len();
        let mut r_best: i32 = 999;

        for l0 in 0..2 {
            let mut t = l0;
            let mut bcnt: i32 = 0;
            for i in 0..c {
                if st[i].o == 1 && st[i].l != t {
                    bcnt += 1;
                    t ^= 1;
                }
                if st[i].o != 1 && st[i].l == t {
                    bcnt += 1;
                    t ^= 1;
                }
            }
            if bcnt < r_best {
                r_best = bcnt;
            }
        }

        outputs.push_str(&format!("{r_best}\n"));
    }

    print!("{outputs}");
}
