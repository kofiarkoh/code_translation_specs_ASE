use std::io::{self, Read};

const FACT8: i64 = 40320;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    while let Some(tok) = it.next() {
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // If an integer cannot be read, processing stops.
        };
        if n == 0 {
            break;
        }

        let ans: i64 = if n <= 16 {
            0
        } else if (n & 1) == 1 {
            // Odd N: count cyclic non-adjacent binary strings of length N with exactly 8 ones.
            // Labeled positions; no quotient by rotation/reflection.
            let nn = n as usize;

            // DP over positions 0..nn-1 with state:
            // pos: next index to decide
            // k: number of ones used so far
            // prev: whether previous position (pos-1) is 1
            // first: whether position 0 is 1 (needed for cyclic adjacency with last)
            //
            // memo[pos][k][prev][first]
            let mut memo = vec![vec![vec![vec![None::<i64>; 2]; 2]; 9]; nn + 1];

            fn rec(
                pos: usize,
                nn: usize,
                k: usize,
                prev: usize,
                first: usize,
                memo: &mut Vec<Vec<Vec<Vec<Option<i64>>>>>,
            ) -> i64 {
                if k > 8 {
                    return 0;
                }
                if pos == nn {
                    // Enforce cyclic adjacency between last and first.
                    if k == 8 && !(prev == 1 && first == 1) {
                        return 1;
                    }
                    return 0;
                }
                if let Some(v) = memo[pos][k][prev][first] {
                    return v;
                }

                let mut total = 0i64;

                // Choose 0 at pos.
                total = total.wrapping_add(rec(pos + 1, nn, k, 0, first, memo));

                // Choose 1 at pos if not adjacent to previous.
                if prev == 0 {
                    let new_first = if pos == 0 { 1 } else { first };
                    total = total.wrapping_add(rec(pos + 1, nn, k + 1, 1, new_first, memo));
                }

                memo[pos][k][prev][first] = Some(total);
                total
            }

            let patterns = rec(0, nn, 0, 0, 0, &mut memo);
            patterns.wrapping_mul(FACT8)
        } else {
            // Even N: paired-half constraint over m = N/2 pairs.
            // Each pair i has state:
            // 0: (0,0)
            // 1: (1,0)  => occupies first half
            // 2: (0,1)  => occupies second half
            // Disallow consecutive 1s, disallow consecutive 2s, cyclically.
            // Total occupied positions across all pairs must be exactly 8.
            let m = (n / 2) as usize;

            // DP over pair index 0..m with state:
            // idx: next pair to decide
            // k: occupied count so far
            // prev: previous pair state (0/1/2)
            // first: first pair state (0/1/2) to enforce cyclic restriction at end
            //
            // memo[idx][k][prev][first]
            let mut memo = vec![vec![vec![vec![None::<i64>; 3]; 3]; 9]; m + 1];

            fn allowed(prev: usize, cur: usize) -> bool {
                // If prev is 1, cur cannot be 1. If prev is 2, cur cannot be 2. prev=0 imposes no restriction.
                if prev == 1 && cur == 1 {
                    return false;
                }
                if prev == 2 && cur == 2 {
                    return false;
                }
                true
            }

            fn rec(
                idx: usize,
                m: usize,
                k: usize,
                prev: usize,
                first: usize,
                memo: &mut Vec<Vec<Vec<Vec<Option<i64>>>>>,
            ) -> i64 {
                if k > 8 {
                    return 0;
                }
                if idx == m {
                    // Enforce cyclic restriction between last (prev) and first.
                    if k == 8 && allowed(prev, first) {
                        return 1;
                    }
                    return 0;
                }
                if let Some(v) = memo[idx][k][prev][first] {
                    return v;
                }

                let mut total = 0i64;

                for cur in 0..=2 {
                    if !allowed(prev, cur) {
                        continue;
                    }
                    let add = if cur == 0 { 0 } else { 1 };
                    let new_first = if idx == 0 { cur } else { first };
                    total = total.wrapping_add(rec(idx + 1, m, k + add, cur, new_first, memo));
                }

                memo[idx][k][prev][first] = Some(total);
                total
            }

            // Start with prev=0 (no restriction before first), first placeholder 0 (will be set at idx==0).
            let patterns = rec(0, m, 0, 0, 0, &mut memo);
            patterns.wrapping_mul(FACT8)
        };

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
