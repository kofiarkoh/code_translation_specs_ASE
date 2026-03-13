// Translated from the provided specification.
// Notes:
// - The specification described three package types with arrays (a, s, c, f) but did not supply concrete values.
// - To produce a runnable translation that preserves the described algorithm and structure, this implementation
//   uses placeholder package parameters below. Replace these constants with the actual values if/when they are known.
// - Behavior otherwise follows the specification:
//   * Precompute minimal costs for totals v = 0..50 (units; each unit corresponds to input n/100).
//   * For each nonzero integer n read from stdin, print the precomputed cost at index floor(n/100).
//   * Stop processing when a zero is read (zero is not answered).
//
// Semantics implemented for package pricing (one reasonable interpretation consistent with the spec):
// - For each package type p (0..2):
//     - a[p] is the number of units that one "pack" of that type supplies.
//     - s[p] is the price of a single pack (per-pack single price).
//     - c[p] is the bundle count (number of packs that form a discounted bundle).
//     - f[p] is the price for such a bundle of c[p] packs.
// - When selecting a nonnegative integer number of packs `count` of type p, the minimal cost for that count
//   is computed by using as many discounted bundles as possible:
//       bundles = count / c[p]
//       remainder_packs = count % c[p]
//       cost_p = bundles * f[p] + remainder_packs * s[p]
// - The precomputation enumerates counts of packs (i, j, k) in 0..=50 for the three types, computes the
//   total units = i*a[0] + j*a[1] + k*a[2], and updates tbl[units] with the minimal total cost found.
// - tbl[0] == 0 by initialization; entries for unreachable totals remain 0.
//
// The implementation is safe and idiomatic Rust. No unsafe code is used.

use std::io::{self, Read};

fn main() {
    // Placeholder package parameters.
    // Replace these with the concrete values if they are available.
    // They must be positive integers for meaningful behavior.
    const A: [i32; 3] = [1, 3, 5]; // base-unit sizes (units per pack)
    const S: [i32; 3] = [2, 5, 9]; // single-pack prices
    const C: [i32; 3] = [3, 5, 2]; // bundle (in packs) sizes
    const F: [i32; 3] = [5, 12, 16]; // bundle prices

    // Table size as per specification: we precompute for indices 0..50 (table allocated a bit larger as in spec)
    const MAX_INDEX: usize = 50;
    const TABLE_SIZE: usize = 60; // as in specification comments (tbl[0..59] exists, but only 0..50 used)

    let mut tbl: [i32; TABLE_SIZE] = [0; TABLE_SIZE]; // zero-initialized

    // Precompute minimal costs for totals v = 0..50.
    // Enumerate number of packs for each of the 3 package types from 0..=50 (bounded loops, small constant).
    for i in 0..=50 {
        for j in 0..=50 {
            for k in 0..=50 {
                // compute total units produced by i,j,k packs of each type
                let units = i as i32 * A[0] + j as i32 * A[1] + k as i32 * A[2];
                if units < 0 {
                    continue; // defensive -- shouldn't happen with nonnegative constants
                }
                let units_usize = units as usize;
                if units_usize > MAX_INDEX {
                    continue;
                }

                // compute cost for each pack count using bundle pricing where beneficial
                let cost_i = (i as i32 / C[0]) * F[0] + (i as i32 % C[0]) * S[0];
                let cost_j = (j as i32 / C[1]) * F[1] + (j as i32 % C[1]) * S[1];
                let cost_k = (k as i32 / C[2]) * F[2] + (k as i32 % C[2]) * S[2];

                let total_cost = cost_i + cost_j + cost_k;

                // Update minimal cost in table:
                // tbl[v] == 0 is used to indicate "no valid combination found" except for v == 0
                // For v == 0, tbl[0] should remain 0 (cost is 0 with zero packs).
                if units_usize == 0 {
                    // For zero units, cost is naturally zero; we don't overwrite it with positive totals.
                    continue;
                }
                if tbl[units_usize] == 0 || total_cost < tbl[units_usize] {
                    tbl[units_usize] = total_cost;
                }
            }
        }
    }

    // Read all integers from stdin (space/newline separated), similar to repeated scanf("%d", &n) until a 0.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        for token in input.split_whitespace() {
            // Parse integer; if parsing fails, mimic the unspecified behavior by skipping the token.
            // The specification did not require explicit handling of parse errors.
            if let Ok(n) = token.parse::<i32>() {
                if n == 0 {
                    break; // termination sentinel
                }
                // Compute index as floor(n/100)
                let idx = (n / 100) as isize;
                if idx < 0 {
                    // Specification assumes non-negative input; for negative, skip (no defined behavior specified).
                    continue;
                }
                let idx_usize = idx as usize;
                // The spec intended indices in 0..=50. For indices > 50 behavior is undefined in the original,
                // but to avoid panics in Rust we treat out-of-range indices as producing no valid combination (print 0).
                // This is a narrow, safe interpretation: print 0 for out-of-domain queries.
                let out = if idx_usize <= MAX_INDEX {
                    tbl[idx_usize]
                } else {
                    0
                };
                println!("{}", out);
            }
        }
    }
}
