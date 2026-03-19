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

        // Interpret positions as 1-based indices in [1, 2^n]. Convert to 0-based.
        // The produced sequence corresponds to the path from the root to the leaf in a
        // conceptual perfect binary partitioning of depth n:
        // - At each level, choose L if the target is in the left half, else R.
        // - If the reference position i is in the right half at that level, the fold
        //   mirrors the partition, swapping L/R for that level and reflecting both i and j
        //   within the current interval.
        //
        // This matches the "folding process determined by a reference position i" and
        // yields the sample outputs.
        let mut ii: i128 = (i as i128) - 1;
        let mut jj: i128 = (j as i128) - 1;

        // Current interval size = 2^n. Use i128 to avoid overflow for moderate n.
        // If n is too large for shifting, behavior is not specified; we clamp by saturating
        // to a very large size to keep deterministic behavior.
        let mut size: i128 = if n >= 0 && n < 120 {
            1i128 << (n as u32)
        } else {
            // Saturate: still allows the loop to run n steps deterministically.
            i128::MAX / 2
        };

        let mut ans = Vec::with_capacity(n as usize);

        for _ in 0..(n as usize) {
            let half = size / 2;

            // Determine whether the reference is in the right half; if so, folding mirrors.
            let mirror = ii >= half;

            if mirror {
                // Reflect both positions within the interval.
                ii = size - 1 - ii;
                jj = size - 1 - jj;
            }

            // Choose based on target position in the (possibly mirrored) interval.
            let mut ch = if jj < half { b'L' } else { b'R' };

            // If we mirrored due to reference being on the right, swap L/R for output.
            if mirror {
                ch = if ch == b'L' { b'R' } else { b'L' };
            }

            ans.push(ch);

            // Descend into the chosen half (in the mirrored coordinate system).
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
