// Translated from C (Aizu Vol-0 0097: Sum of Integers II).
// Preserves C semantics (including 32-bit signed wraparound) by using wrapping arithmetic.
//
// Notes:
// - The original used a global int p[10][101][902]; we use i32 and emulate signed 32-bit wraparound
//   with wrapping_add to match typical C two's-complement behavior on overflow.
// - I/O behavior (formatting, newlines) is preserved.

use std::io::{self, Read};

fn main() {
    // p[n][i][j] same shape as C: [10][101][902]
    // Use i32 and wrapping_add to emulate C signed int wraparound behavior.
    let mut p = [[[0i32; 902]; 101]; 10];

    // Initialize base case: for (i = 0; i <= 100; i++) p[1][i][i] = 1;
    for i in 0..=100 {
        p[1][i][i] = 1;
    }

    // Build DP as in C:
    // for (min = 0, max = 100, n = 1; n < 9; n++, min++, max+=100)
    //   for (i = 0; i <= 100; i++)
    //     for (j = min; j <= max; j++)
    //       for (k = 1; k + i <= 100; k++) p[n+1][i+k][j+i+k] += p[n][i][j];
    let mut min: usize = 0;
    let mut max: usize = 100;
    for n in 1usize..9usize {
        for i in 0usize..=100usize {
            let k_max = 100usize.saturating_sub(i);
            for j in min..=max {
                // read value once
                let val = p[n][i][j];
                if val == 0 {
                    continue;
                }
                // k from 1 to k_max inclusive
                for k in 1..=k_max {
                    // indices computed as usize
                    let ni = i + k; // <=100
                    let nj = j + i + k;
                    // Safe to index because array sizes were chosen to match C.
                    p[n + 1][ni][nj] = p[n + 1][ni][nj].wrapping_add(val);
                }
            }
        }
        min += 1;
        max += 100;
    }

    // Read all input tokens as integers
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace().map(|s| s.parse::<i32>());

    // Process pairs (n, s) similarly to the C loop: while (scanf("%d%d", &n, &s) && n) { ... }
    // We take pairs from tokens; if odd number of tokens, trailing one is ignored.
    while let (Some(Ok(n)), Some(Ok(s))) = (tokens.next(), tokens.next()) {
        if n == 0 {
            break;
        }
        if s > 864 {
            // puts("0") in C prints "0\n"
            println!("0");
        } else {
            // sum ans over i=0..100: ans += p[n][i][s];
            // Emulate C signed int wraparound using wrapping_add.
            let mut ans: i32 = 0;
            // n used as index into first dimension; ensure conversion
            let ni = n as usize;
            let sj = s as usize;
            for i in 0usize..=100usize {
                ans = ans.wrapping_add(p[ni][i][sj]);
            }
            println!("{}", ans);
        }
    }
}
