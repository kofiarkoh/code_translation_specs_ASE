use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity per spec (at most 10^6). If exceeded, behavior is undefined;
    // we still allocate as requested to keep the program compilable.
    let mut a: Vec<u8> = vec![0u8; n];

    let mut total: u64 = 0;

    for i in 0..m {
        for j in 0..n {
            let v: u8 = it.next().unwrap().parse::<u8>().unwrap();
            // Values are assumed to be 0/1; other values are outside the meaningful contract.
            a[j] = if i != 0 { v ^ 1 } else { v };
        }

        if n == 0 {
            continue;
        }

        let mut l: isize = 0;
        let mut r: isize = (n as isize) - 1;

        while l <= r {
            let li = l as usize;
            let ri = r as usize;

            if a[li] == 1 {
                total += 1;
                l += 1;
                // Per spec: toggle at the new boundary position even if l has moved past r.
                // In the original C-like semantics, this could be out-of-bounds (undefined).
                // Here we emulate the intended in-bounds behavior when possible; otherwise we do nothing.
                if l >= 0 && (l as usize) < n {
                    let idx = l as usize;
                    a[idx] ^= 1;
                }
            } else if a[ri] == 1 {
                total += 1;
                r -= 1;
                // Same note as above regarding potential out-of-bounds.
                if r >= 0 && (r as usize) < n {
                    let idx = r as usize;
                    a[idx] ^= 1;
                }
            } else {
                l += 1;
                if l >= 0 && (l as usize) < n {
                    let idx = l as usize;
                    a[idx] ^= 1;
                }
            }
        }
    }

    println!("{total}");
}
