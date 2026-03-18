use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_mul(a: u64, b: u64) -> u64 {
    (a * b) % MOD
}

fn main() {
    // This matches the well-known combinatorial rule for comparing two equal-length strings:
    // Scan left-to-right, grouping positions into either:
    // - Vertical domino: S1[i] == S2[i] (uses one column)
    // - Horizontal domino pair: S1[i] != S2[i] and thus must pair with i+1 (uses two columns)
    //
    // Count the number of valid colorings under constraints:
    // - For a vertical domino, its two cells share the same color.
    // - For a horizontal pair (two dominoes stacked), each domino spans two columns.
    // - Adjacent dominoes (sharing an edge) must have different colors.
    //
    // The resulting count is computed modulo MOD.
    //
    // Transition multipliers:
    // - First piece:
    //   - vertical: 3
    //   - horizontal: 6
    // - Next piece:
    //   - prev vertical -> vertical: *2
    //   - prev vertical -> horizontal: *2
    //   - prev horizontal -> vertical: *1
    //   - prev horizontal -> horizontal: *3
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s1 = it.next().unwrap().as_bytes().to_vec();
    let s2 = it.next().unwrap().as_bytes().to_vec();

    let mut i = 0usize;
    let mut ans: u64 = 1;
    // prev = 0 => none yet, 1 => previous was vertical, 2 => previous was horizontal
    let mut prev: u8 = 0;

    while i < n {
        if s1[i] == s2[i] {
            // vertical
            if prev == 0 {
                ans = mod_mul(ans, 3);
            } else if prev == 1 {
                ans = mod_mul(ans, 2);
            } else {
                // prev horizontal
                ans = mod_mul(ans, 1);
            }
            prev = 1;
            i += 1;
        } else {
            // horizontal pair (requires i+1)
            if prev == 0 {
                ans = mod_mul(ans, 6);
            } else if prev == 1 {
                ans = mod_mul(ans, 2);
            } else {
                // prev horizontal
                ans = mod_mul(ans, 3);
            }
            prev = 2;
            i += 2;
        }
    }

    println!("{}", ans % MOD);
}
