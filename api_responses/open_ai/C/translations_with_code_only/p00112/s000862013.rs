// Translated from the provided C program.
// Preserves semantics as closely as practical in safe Rust.
// Notable choices:
// - C used `long long int` (typically 64-bit). We use i64.
// - The C swap used arithmetic operations which, on typical two's-complement machines,
//   exhibit wraparound on overflow. To emulate that behavior (and avoid UB differences),
//   we use wrapping_add / wrapping_sub so the bit-level wraparound is preserved.
// - The selection sort's tie-breaking chooses the later index when equal
//   (because the C code used `time[min] < time[k] ? min : k`), and we preserve that.
// - Input parsing: we read all whitespace-separated tokens and process them similarly
//   to repeated scanf calls. If parsing fails or input ends early, processing stops.

use std::io::{self, Read};

fn main() {
    // Arrays of size 10001 initialized to zero, like `long long int time[10001] = {}` in C.
    const MAXN: usize = 10001;
    let mut time: Vec<i64> = vec![0; MAXN];
    let mut wtime: Vec<i64> = vec![0; MAXN];

    // Read all input tokens
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read failed");
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut idx: usize = 0;

    // Emulate: while (scanf("%d", &n) && n) { ... }
    loop {
        // Read n (if no more tokens, stop)
        if idx >= tokens.len() {
            break;
        }
        let n_parsed = tokens[idx].parse::<i64>();
        idx += 1;
        let n = match n_parsed {
            Ok(v) => v,
            Err(_) => break, // scan failure -> stop like scanf would
        };

        if n == 0 {
            break; // matches `&& n` condition: stop when n is zero
        }

        // Read `n` time values if n > 0. If n <= 0, C's for(j=0;j<n;j++) won't run.
        if n > 0 {
            let count = n as usize;
            // Ensure we don't panic on insufficient tokens; mimic scanf blocking/EOF by stopping.
            if idx + count > tokens.len() {
                break;
            }
            for j in 0..count {
                // parse as i64, on parse failure stop processing.
                match tokens[idx].parse::<i64>() {
                    Ok(val) => time[j] = val,
                    Err(_) => return,
                }
                idx += 1;
            }
        }
        // If n <= 0, we do not read any times (matches C behavior).

        // Selection sort with the exact comparison behavior:
        // min = time[min] < time[k] ? min : k
        // Note: use i64 indices for comparison with possibly negative n; when indexing we cast.
        let mut j: i64 = 0;
        while j < n - 1 {
            let mut min = j;
            let mut k = j + 1;
            while k < n {
                let minu = min as usize;
                let ku = k as usize;
                // If time[min] < time[k], keep min; else take k.
                if time[minu] < time[ku] {
                    // min stays
                } else {
                    min = k;
                }
                k += 1;
            }
            if min != j {
                let ju = j as usize;
                let minu = min as usize;
                // Perform the same three-step arithmetic swap, using wrapping ops to emulate
                // typical two's-complement wraparound behavior.
                time[minu] = time[minu].wrapping_add(time[ju]);
                time[ju] = time[minu].wrapping_sub(time[ju]);
                time[minu] = time[minu].wrapping_sub(time[ju]);
            }
            j += 1;
        }

        // Compute waiting times and their sum. Use wrapping arithmetic to emulate C's behavior.
        let mut sum: i64 = 0;
        let mut jj: i64 = 1;
        while jj < n {
            let j_u = jj as usize;
            let jm1_u = (jj - 1) as usize;
            wtime[j_u] = wtime[jm1_u].wrapping_add(time[jm1_u]);
            sum = sum.wrapping_add(wtime[j_u]);
            jj += 1;
        }

        // Print sum like printf("%lld\n", sum);
        println!("{}", sum);
    }
}
