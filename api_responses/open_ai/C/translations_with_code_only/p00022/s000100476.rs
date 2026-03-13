use std::io::{self, Read};

const INT_MIN: i32 = -1_000_000_000; // matches C's const int INT_MIN = -1e9;

fn main() {
    // Read all stdin as whitespace-separated tokens (like repeated scanf("%d", ...)).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace().map(|s| s.parse::<i32>().expect("parse int"));

    // Fixed-size arrays to mirror the C program's static arrays.
    // Note: indexing beyond these sizes will panic in Rust (safe bound checks).
    // The original C code would have undefined behavior in that case.
    let mut a = [0i32; 5000];
    let mut left = [0i32; 5010];
    let mut right = [0i32; 5010];

    // Loop over test cases: while(scanf("%d",&n), n) in C means read n and stop when n==0.
    while let Some(n) = it.next() {
        if n == 0 {
            break;
        }
        let n_usize = n as usize;

        // Zero out arrays like the C code does with memset.
        // (They are already zeroed at start; do so again to match per-test-case behavior.)
        for v in a.iter_mut().take(5000) {
            *v = 0;
        }
        for v in left.iter_mut().take(5010) {
            *v = 0;
        }
        for v in right.iter_mut().take(5010) {
            *v = 0;
        }

        // Read n integers into a[0..n-1]
        for i in 0..n_usize {
            // If input is insufficient, this will panic, similar to how scanf would fail.
            a[i] = it.next().expect("expected more integers for array a");
        }

        // Compute left prefixes: left[i] = max(left[i-1] + a[i-1], 0), left[0]=0
        if n_usize > 0 {
            left[0] = 0;
            for i in 1..n_usize {
                let sum = left[i - 1].wrapping_add(a[i - 1]); // emulate C 32-bit wraparound if it occurs
                left[i] = if sum > 0 { sum } else { 0 };
            }

            // Compute right suffixes: right[i] = max(right[i+1] + a[i+1], 0), right[n-1]=0
            right[n_usize - 1] = 0;
            if n_usize >= 2 {
                for i in (0..n_usize - 1).rev() {
                    let sum = right[i + 1].wrapping_add(a[i + 1]); // preserve wraparound semantics
                    right[i] = if sum > 0 { sum } else { 0 };
                }
            }

            // Compute answer: max over i of left[i] + a[i] + right[i]
            let mut ans: i32 = INT_MIN;
            for i in 0..n_usize {
                // Use wrapping_add to emulate C's signed integer wraparound in sums.
                let s = left[i]
                    .wrapping_add(a[i])
                    .wrapping_add(right[i]);
                ans = if ans > s { ans } else { s };
            }

            println!("{}", ans);
        } else {
            // n == 0 case is handled above by breaking; negative n would reach here in C and
            // likely cause UB. Here, we avoid further action for n==0 (shouldn't happen).
        }
    }
}
