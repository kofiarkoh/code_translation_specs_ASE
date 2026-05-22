use std::io::{self, Read};

const PRIME: i64 = 10_000_000_019;
const K_MAX: i64 = 2_222_222;

fn calcul(mut p: i64, mut q: i64, mut z: i64) -> (i64, i64, i64) {
    // Returns (cnt, outR, outB) following the specification exactly.
    let mut swapped = false;
    if p < q {
        std::mem::swap(&mut p, &mut q);
        swapped = true;
    }

    let mut cnt: i64 = 0;
    let mut out_r: i64 = 0;
    let mut out_b: i64 = 0;

    let mut i: i64 = 0;
    while z >= 0 {
        let w = z / q; // integer division
        let wp1 = w + 1;

        cnt = cnt + wp1;
        out_r = out_r + i * wp1;
        out_b = out_b + (wp1 * w) / 2;

        z = z - p;
        i += 1;
    }

    if swapped {
        std::mem::swap(&mut out_r, &mut out_b);
    }
    (cnt, out_r, out_b)
}

fn calc(p: i64, q: i64, k: i64) -> (i64, i64) {
    // Binary search z in [0, PRIME*2000], keeping the last calcul outputs.
    let mut lo: i64 = 0;
    let mut hi: i64 = PRIME * 2000;

    let mut last_r: i64 = 0;
    let mut last_b: i64 = 0;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        let (c, r, b) = calcul(p, q, mid);
        last_r = r;
        last_b = b;

        if c == k {
            break;
        } else if c < k {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    (last_r, last_b)
}

fn ispossible(r: i64, b: i64, k: i64) -> bool {
    // Binary search te in [1, PRIME-1] with early exits and final cross-product test.
    let mut lo: i64 = 1;
    let mut hi: i64 = PRIME - 1;

    // Bounding points:
    // F from a case where Z_R <= R, S from a case where Z_R > R.
    let mut fr: i64 = 0;
    let mut fb: i64 = 0;
    let mut sr: i64 = 0;
    let mut sb: i64 = 0;

    while lo <= hi {
        let te = lo + (hi - lo) / 2;
        let (zr, zb) = calc(te, PRIME - te, k);

        if zr <= r && zb <= b {
            return true;
        }
        if zr > r && zb > b {
            return false;
        }

        if zr > r {
            // Move upward in te
            sr = zr;
            sb = zb;
            lo = te + 1;
        } else {
            // zr <= r but zb > b (since not returned above)
            fr = zr;
            fb = zb;
            hi = te - 1;
        }
    }

    // Final geometric check: cross((S-F),(T-F)) >= 0
    // cross = (sr-fr)*(b-fb) - (sb-fb)*(r-fr)
    let cross = (sr - fr) * (b - fb) - (sb - fb) * (r - fr);
    cross >= 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let r: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Binary search maximum k in [0, K_MAX] such that ispossible(r,b,k) is true.
    let mut lo: i64 = 0;
    let mut hi: i64 = K_MAX;
    let mut best: i64 = 0;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if ispossible(r, b, mid) {
            best = mid;
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    // Always subtract 1 as specified.
    let ans = best - 1;
    println!("{ans}");
}
