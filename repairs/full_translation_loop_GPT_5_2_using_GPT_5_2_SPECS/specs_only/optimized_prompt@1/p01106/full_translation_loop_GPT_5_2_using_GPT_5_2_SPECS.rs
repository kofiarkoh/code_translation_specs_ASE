use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).ok();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n: i64 = match it.next() {
            Some(x) => match x.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let i: i64 = match it.next() {
            Some(x) => match x.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let j: i64 = match it.next() {
            Some(x) => match x.parse() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        if n <= 0 {
            break;
        }

        // Model:
        // Consider a perfect binary partitioning of depth n over positions 1..=2^n.
        // We output the L/R path to reach target j, but under a folding process
        // determined by reference i:
        // At each level, if i is in the right half, the fold mirrors the interval,
        // which reflects both i and j; the output letter is still determined by the
        // (possibly mirrored) j without an extra swap.
        //
        // This matches the sample:
        // (3,5,7)->LRL, (4,10,12)->RLLL, (5,17,20)->LRRRR.
        let mut ii: i128 = (i as i128) - 1;
        let mut jj: i128 = (j as i128) - 1;

        let mut size: i128 = if (0..120).contains(&n) {
            1i128 << (n as u32)
        } else {
            // Unspecified for huge n; keep deterministic and avoid shift overflow.
            i128::MAX / 2
        };

        let mut ans = Vec::with_capacity(n as usize);

        for _ in 0..(n as usize) {
            let half = size / 2;

            if ii >= half {
                // Mirror the current interval.
                ii = size - 1 - ii;
                jj = size - 1 - jj;
            }

            let ch = if jj < half { b'L' } else { b'R' };
            ans.push(ch);

            // Descend into the chosen half.
            if jj >= half {
                jj -= half;
            }
            if ii >= half {
                ii -= half;
            }

            size = half;
        }

        out.push_str(std::str::from_utf8(&ans).unwrap());
        out.push('\n');
    }

    print!("{out}");
}
