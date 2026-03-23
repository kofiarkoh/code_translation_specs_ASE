use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let m: usize = it.next().unwrap().parse().unwrap();
    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed internal capacity per spec/C code (A[106]).
    // If n > 106, behavior is undefined per spec; we do not add explicit error handling.
    let mut a: [i32; 106] = [0; 106];

    let mut total: i32 = 0;

    for i in 0..m {
        for j in 0..n {
            let mut v: i32 = it.next().unwrap().parse().unwrap();
            if i != 0 {
                v ^= 1;
            }
            a[j] = v;
        }

        // Per-row two-ended stateful procedure.
        // Note: The spec (and the C code) toggles A[l] after incrementing l (or A[r] after decrementing r)
        // even if the index has moved past the other boundary; in C this can go out of bounds when l==n-1
        // and A[l]==1 (it toggles A[n]). We preserve the intended behavior for in-bounds indices and
        // otherwise do nothing (Rust cannot perform out-of-bounds access safely).
        let mut l: isize = 0;
        let mut r: isize = n as isize - 1;

        while l <= r {
            let li = l as usize;
            let ri = r as usize;

            if a[li] == 1 {
                total += 1;
                l += 1;
                if l >= 0 && (l as usize) < n {
                    a[l as usize] ^= 1;
                }
            } else if a[ri] == 1 {
                total += 1;
                r -= 1;
                if r >= 0 && (r as usize) < n {
                    a[r as usize] ^= 1;
                }
            } else {
                l += 1;
                if l >= 0 && (l as usize) < n {
                    a[l as usize] ^= 1;
                }
            }
        }
    }

    print!("{total}\n");
}
