use std::io::{self, Read};

fn popcount_u32(x: u32) -> u32 {
    x.count_ones()
}

fn main() {
    // Read entire stdin; undefined behavior for malformed input per spec, so we keep parsing simple.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Number of sets (non-negative). If missing, do nothing (narrowest reasonable behavior).
    let t_opt = it.next();
    if t_opt.is_none() {
        return;
    }
    let t: usize = t_opt.unwrap().parse().unwrap();

    // Fixed memory constraints: up to 20 sets, 20 pairs per set (per spec).
    // We still store dynamically but do not assume more than that.
    let mut sets: Vec<Vec<(u32, u32)>> = Vec::with_capacity(t);

    for _ in 0..t {
        let m: usize = it.next().unwrap().parse().unwrap();
        let mut pairs: Vec<(u32, u32)> = Vec::with_capacity(m);
        for _ in 0..m {
            let a: u32 = it.next().unwrap().parse().unwrap();
            let b: u32 = it.next().unwrap().parse().unwrap();
            pairs.push((a, b));
        }
        sets.push(pairs);
    }

    // Interpretation (narrowest reasonable given spec):
    // - We choose exactly one integer from each pair (either first or second).
    // - A combination is "valid" if for every set, the chosen integers across its pairs are all distinct.
    //   (This is a common "validity based on pairs" constraint; spec is otherwise ambiguous.)
    // - For a valid combination, compute the bitwise OR of all chosen integers across all sets/pairs,
    //   and count set bits; maximize this count.
    //
    // This yields a single integer output: maximum number of set bits among valid combinations.
    //
    // Note: If a set has 0 pairs, it contributes nothing and is always valid.

    // Precompute per set all valid OR-results achievable within that set, along with their popcount.
    // Then combine sets by OR-ing their results and maximizing popcount.
    let mut per_set_ors: Vec<Vec<u32>> = Vec::with_capacity(sets.len());

    for pairs in &sets {
        let m = pairs.len();
        if m == 0 {
            per_set_ors.push(vec![0]);
            continue;
        }

        // Enumerate all 2^m choices within the set.
        // Validity: chosen values must be distinct within the set.
        let mut ors: Vec<u32> = Vec::new();
        let total = 1usize << m;
        for mask in 0..total {
            let mut ok = true;
            let mut chosen: Vec<u32> = Vec::with_capacity(m);
            let mut acc_or: u32 = 0;
            for (i, &(a, b)) in pairs.iter().enumerate() {
                let v = if ((mask >> i) & 1) == 0 { a } else { b };
                // Enforce distinctness within the set.
                if chosen.iter().any(|&x| x == v) {
                    ok = false;
                    break;
                }
                chosen.push(v);
                acc_or |= v;
            }
            if ok {
                ors.push(acc_or);
            }
        }

        // If no valid choices exist, the set makes the whole combination impossible.
        // Narrowest behavior: keep empty list; final answer will remain 0.
        per_set_ors.push(ors);
    }

    // Combine sets: DP over OR values.
    // Since integers are u32, OR-space can be large; we keep a map-like set via Vec and dedup.
    // Given constraints (<=20 sets, <=20 pairs), this may still be large; spec notes exponential growth.
    let mut current: Vec<u32> = vec![0];

    for ors in per_set_ors {
        if ors.is_empty() {
            // No valid choices for this set => no valid global combinations.
            current.clear();
            break;
        }
        let mut next: Vec<u32> = Vec::new();
        next.reserve(current.len().saturating_mul(ors.len()));
        for &c in &current {
            for &o in &ors {
                next.push(c | o);
            }
        }
        // Deduplicate to control growth somewhat.
        next.sort_unstable();
        next.dedup();
        current = next;
    }

    let mut best: u32 = 0;
    for v in current {
        best = best.max(popcount_u32(v));
    }

    // Output a single integer.
    println!("{}", best);
}
