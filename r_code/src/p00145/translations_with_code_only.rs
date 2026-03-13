use std::io::{self, Read};

fn main() {
    // Read all stdin and parse whitespace-separated integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut it = input.split_whitespace();

    // Helper to parse next token as i32. If parsing fails or token missing, panic,
    // which mirrors the C program's expectation of well-formed input.
    let next_i32 = |it: &mut std::str::SplitWhitespace| -> i32 {
        it.next()
            .expect("missing input")
            .parse::<i32>()
            .expect("bad integer")
    };

    let n = next_i32(&mut it) as usize;

    // In the original C code c1 and c2 are arrays of size 100 and the code
    // accesses index `n` (one past the last real element) in the shift loop.
    // To avoid out-of-bounds in Rust while staying close to the original behavior,
    // allocate vectors of length n+1 and initialize the extra slot to 0.
    //
    // Note: the original C had other potential undefined behaviors (e.g. an
    // uninitialized `amaxi`). We initialize `amaxi` here to 0 to avoid UB in Rust;
    // this may differ from some pathological runs of the C program that relied on
    // indeterminate stack contents.
    let mut c1 = vec![0i32; n + 1];
    let mut c2 = vec![0i32; n + 1];

    for i in 0..n {
        c1[i] = next_i32(&mut it);
        c2[i] = next_i32(&mut it);
    }

    // All arithmetic in the C program used `int`. To emulate typical two's-complement
    // wraparound behavior (and avoid Rust debug-overflow panics), use wrapping ops.
    let mut s: i32 = 0;
    let mut amax: i32 = 0;
    let mut amaxi: usize = 0; // Initialized to 0 to avoid undefined behavior from C's uninitialized use

    for i in 0..(n.saturating_sub(1)) {
        // inner loop: for(j=0;j<n-i-1;j++)
        let limit = n - i - 1;
        for j in 0..limit {
            // a = c2[j] * c1[j+1];
            let a = c2[j].wrapping_mul(c1[j + 1]);
            // Compare as signed i32 values (using wrapped multiplication results).
            // Tie-break uses c1[j]*c2[j+1] < c1[amaxi]*c2[amaxi+1]
            let take_j = if a > amax {
                true
            } else if a == amax {
                // compute tie-break products with wrapping mul
                let lhs = c1[j].wrapping_mul(c2[j + 1]);
                let rhs = c1[amaxi].wrapping_mul(c2[amaxi + 1]);
                lhs < rhs
            } else {
                false
            };

            if take_j {
                amax = a;
                amaxi = j;
            }
        }

        // s += c1[amaxi]*c2[amaxi]*c1[amaxi+1]*c2[amaxi+1];
        let prod = c1[amaxi]
            .wrapping_mul(c2[amaxi])
            .wrapping_mul(c1[amaxi + 1])
            .wrapping_mul(c2[amaxi + 1]);
        s = s.wrapping_add(prod);

        // c2[amaxi]=c2[amaxi+1];
        c2[amaxi] = c2[amaxi + 1];

        // for(j=amaxi+1;j<n-i;j++){ c1[j]=c1[j+1]; c2[j]=c2[j+1]; }
        let shift_limit = n - i;
        let mut j = amaxi + 1;
        while j < shift_limit {
            // accesses up to index `n` (when j == n-1, j+1 == n). We've allocated n+1 slots.
            c1[j] = c1[j + 1];
            c2[j] = c2[j + 1];
            j += 1;
        }

        amax = 0;
        // Note: amaxi left as-is; next iteration it will be overwritten when appropriate.
    }

    // Print result with newline, preserving formatting.
    println!("{}", s);
}
