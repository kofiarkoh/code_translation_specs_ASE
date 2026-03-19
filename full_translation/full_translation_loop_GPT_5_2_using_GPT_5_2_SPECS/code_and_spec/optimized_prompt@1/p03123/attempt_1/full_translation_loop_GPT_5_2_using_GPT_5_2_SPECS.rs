use std::io::{self, Read};

const PRIME: i64 = 10_000_000_019;

fn llsankaku(x: i64) -> i64 {
    ((1 + x) * x) / 2
}

fn calcul(mut p: i64, mut q: i64, mut z: i64) -> (i64, i64, i64) {
    // returns (cnt, r, b)
    let mut f = false;
    let mut cnt: i64 = 0;
    let mut r: i64 = 0;
    let mut b: i64 = 0;

    if p < q {
        f = true;
        std::mem::swap(&mut p, &mut q);
    }

    let mut i: i64 = 0;
    while z >= 0 {
        let w = z / q;
        cnt += w + 1;
        r += i * (w + 1);
        b += llsankaku(w);
        z -= p;
        i += 1;
    }

    if f {
        std::mem::swap(&mut r, &mut b);
    }
    (cnt, r, b)
}

fn calc(p: i64, q: i64, k: i64) -> (i64, i64) {
    // Binary search te such that calcul(p,q,te).cnt == k, then return (r,b) from that te.
    // Mirrors C behavior: if no exact te exists, the loop ends with last computed (r,b),
    // but the C code relies on monotonicity and existence in its usage.
    let mut st: i64 = 0;
    let mut fi: i64 = PRIME * 2000;
    let mut last_r: i64 = 0;
    let mut last_b: i64 = 0;

    while st <= fi {
        let te = (st + fi) / 2;
        let (c, r, b) = calcul(p, q, te);
        last_r = r;
        last_b = b;

        if c == k {
            break;
        }
        if c < k {
            st = te + 1;
        }
        if c > k {
            fi = te - 1;
        }
    }
    (last_r, last_b)
}

#[derive(Clone, Copy)]
struct Point {
    xz: i64,
    yz: i64,
}

fn vector_minus(a: Point, b: Point) -> Point {
    Point {
        xz: a.xz - b.xz,
        yz: a.yz - b.yz,
    }
}

fn crossproduct(a: Point, b: Point) -> i64 {
    a.xz * b.yz - a.yz * b.xz
}

fn ispossible(r: i64, b: i64, k: i64) -> bool {
    let mut st: i64 = 1;
    let mut fi: i64 = PRIME - 1;

    // NOTE: The original C has potential undefined behavior: fr/fb/sr/sb may be read
    // uninitialized if the loop exits without setting them on both sides.
    // We preserve the intended control flow but initialize to 0 to keep Rust defined.
    // This may differ from C in those UB cases; spec allows UB there.
    let mut fr: i64 = 0;
    let mut fb: i64 = 0;
    let mut sr: i64 = 0;
    let mut sb: i64 = 0;

    while st <= fi {
        let te = (st + fi) / 2;
        let (zr, zb) = calc(te, PRIME - te, k);

        if zr <= r && zb <= b {
            return true;
        }
        if zr > r && zb > b {
            return false;
        }
        if zr > r {
            st = te + 1;
            sr = zr;
            sb = zb;
        } else {
            fi = te - 1;
            fr = zr;
            fb = zb;
        }
    }

    // Is (r,b) on/above the line through (fr,fb)->(sr,sb)?
    let tg = Point { xz: r, yz: b };
    let l1 = Point { xz: fr, yz: fb };
    let l2 = Point { xz: sr, yz: sb };
    crossproduct(vector_minus(l2, l1), vector_minus(tg, l1)) >= 0
}

fn solve(r: i64, b: i64) -> i64 {
    let mut st: i64 = 0;
    let mut fi: i64 = 2_222_222;
    while st <= fi {
        let te = (st + fi) / 2;
        if ispossible(r, b, te) {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    fi
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let r: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // -1 : empty box (matches C)
    let ans = solve(r, b) - 1;
    print!("{ans}\n");
}
