// translated.rs
// Translated from the provided C program.
// Notes on semantics preservation:
// - C used `long long` for data array entries: mapped to i64.
// - To emulate typical two's-complement behavior for multiplication/addition
//   (since C signed overflow is undefined but many implementations wrap),
//   we use wrapping_mul/wrapping_add on i64 to preserve bit-level wraparound.
// - The program assumes inputs reference numbers in 1..=4000 like the C code.
// - I/O formatting (whitespace/newlines) is preserved.

use std::io::{self, Read};

fn main() {
    // Mirror C arrays: data[4000][4] as Vec<[i64;4]>, kaunto[4000][2] as Vec<[i32;2]>
    let mut data = vec![[0i64; 4]; 4000];
    let mut kaunto = vec![[0i32; 2]; 4000];

    for i in 0..4000 {
        data[i][0] = (i as i64) + 1;
        kaunto[i][0] = (i as i32) + 1;
    }
    // data[*][1..3] and kaunto[*][1] are already zero-initialized.

    // Read entire stdin and split into tokens like scanf("%d" / "%lld")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut iter = input.split_whitespace();

    // Helper closures to parse next token into required integer types.
    let mut next_i32 = || -> Option<i32> {
        iter.next().and_then(|s| s.parse::<i32>().ok())
    };
    let mut next_usize = || -> Option<usize> {
        iter.next().and_then(|s| s.parse::<usize>().ok())
    };
    let mut next_i64 = || -> Option<i64> {
        iter.next().and_then(|s| s.parse::<i64>().ok())
    };

    // Read first kazu
    while let Some(kazu) = next_i32() {
        if kazu == 0 {
            break;
        }
        let mut count = 0i32;

        for _ in 0..kazu {
            // In the original C code, number is read as int then used as index (number-1).
            // We parse it as usize and assume inputs are valid 1..=4000 as in the C program.
            let number_opt = next_usize();
            let a_opt = next_i64();
            let b_opt = next_i64();

            // If any of the expected tokens are missing, mimic C behavior by stopping.
            // (C would have undefined behaviour/blocked scanf; here we just stop processing.)
            let (number, a, b) = match (number_opt, a_opt, b_opt) {
                (Some(n), Some(a), Some(b)) => (n, a, b),
                _ => return,
            };

            // Convert to 0-based index; indexing assumes valid range as in C.
            let idx = number - 1;
            if idx >= data.len() {
                // If out-of-range, to avoid panic we simply continue (best-effort).
                // The original C behavior is undefined; typical inputs should be valid.
                continue;
            }

            // Store the values as in C: data[number-1][1] and [2]
            data[idx][1] = a;
            data[idx][2] = b;
            // data[number-1][3] += data[number-1][1] * data[number-1][2];
            // Use wrapping operations to emulate two's-complement wraparound.
            let prod = data[idx][1].wrapping_mul(data[idx][2]);
            data[idx][3] = data[idx][3].wrapping_add(prod);
        }

        for i in 0..4000 {
            if data[i][3] >= 1_000_000 && kaunto[i][1] == 0 {
                // Print kaunto[i][0] exactly as C does ("%d\n")
                println!("{}", kaunto[i][0]);
                kaunto[i][1] = 1;
                count = 1;
            }
        }
        if count == 0 {
            println!("NA");
        }
        // Next kazu will be read by the loop condition (attempted above).
        // But since we used a while let loop reading kazu at the top, continue to next iteration.
        // The next iteration will take the next token as `kazu`.
    }
}
