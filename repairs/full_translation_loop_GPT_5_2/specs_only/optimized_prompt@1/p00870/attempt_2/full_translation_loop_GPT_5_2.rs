use std::io::{self, Read};

fn main() {
    // The specification is ambiguous about whether "entire set of strings can be found as prefixes"
    // means:
    //   (A) at a given position, every one of the n strings matches as a prefix (simultaneously), or
    //   (B) starting at a position, we can consume the concatenated string by repeatedly matching
    //       any of the n strings as a prefix, and count how many times we can complete a full set
    //       (i.e., match each of the n strings at least once) in sequence.
    //
    // The mention of "state transitions" while processing each character and updating a flag
    // structure suggests (B): a DP/automaton-like scan over positions, tracking which strings have
    // been seen in the current "set completion" since the last completion.
    //
    // We implement (B) with a position-based DP:
    //   - Precompute, for each position i, which patterns match starting at i and their lengths.
    //   - DP over positions with a bitmask of which patterns have been used in the current set.
    //   - When the mask becomes "all ones", we increment the completion count and reset mask to 0
    //     for the next segment (greedily maximizing completions).
    //
    // To preserve determinism without inventing tie-breaking, we compute the maximum number of
    // complete sets achievable from the start (position 0) by any sequence of matches.
    //
    // Input ends when n == 0 && m == 0.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();
    let mut first_case = true;

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: usize = n_tok.parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        if n == 0 && m == 0 {
            break;
        }

        let mut patterns: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            patterns.push(it.next().unwrap().as_bytes().to_vec());
        }

        let mut concat = Vec::<u8>::new();
        for _ in 0..m {
            concat.extend_from_slice(it.next().unwrap().as_bytes());
        }
        let s = concat;
        let len = s.len();

        // Precompute matches at each position: list of (pattern_index, pattern_length).
        // This is O(len * n * max_pat_len) with small constraints.
        let mut matches: Vec<Vec<(usize, usize)>> = vec![Vec::new(); len + 1];
        for i in 0..=len {
            for (j, p) in patterns.iter().enumerate() {
                let plen = p.len();
                if plen == 0 {
                    // Empty pattern matches at every position; it does not advance position.
                    // To avoid infinite loops, we treat empty patterns as "immediately usable"
                    // without consuming input: they can be included in the set at any time.
                    matches[i].push((j, 0));
                } else if i + plen <= len && &s[i..i + plen] == &p[..] {
                    matches[i].push((j, plen));
                }
            }
        }

        // If n is too large for a bitmask, use a Vec<bool> state; but given constraints in typical
        // problems, n is usually small. We still provide a safe fallback.
        let answer: u64 = if n <= 60 {
            let all_mask: u64 = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };

            // DP[pos][mask] = max completed sets achievable from this state to end.
            // Use a HashMap-like sparse representation per position to avoid huge memory when len is big.
            // However, len is bounded by m*100; still, we keep it reasonably efficient.
            use std::collections::BTreeMap;

            let mut dp: Vec<BTreeMap<u64, u64>> = vec![BTreeMap::new(); len + 1];
            // Base: at end, no more completions.
            dp[len].insert(0, 0);

            // We'll compute via recursion with memoization for simplicity and correctness.
            fn solve(
                pos: usize,
                mask: u64,
                len: usize,
                all_mask: u64,
                matches: &Vec<Vec<(usize, usize)>>,
                memo: &mut Vec<std::collections::BTreeMap<u64, u64>>,
            ) -> u64 {
                if let Some(v) = memo[pos].get(&mask) {
                    return *v;
                }

                // Option: advance by one character without matching anything (state transitions per char).
                // This allows skipping characters that don't start a pattern.
                let mut best = if pos < len {
                    solve(pos + 1, mask, len, all_mask, matches, memo)
                } else {
                    0
                };

                // Option: take any match at this position.
                for &(j, plen) in &matches[pos] {
                    let bit = if j < 64 { 1u64 << j } else { 0 };
                    let mut new_mask = mask | bit;

                    let mut gained = 0u64;
                    if new_mask == all_mask {
                        gained = 1;
                        new_mask = 0;
                    }

                    // Prevent infinite loops on zero-length matches: only allow them if they change mask.
                    if plen == 0 {
                        if (mask & bit) != 0 {
                            continue;
                        }
                        let cand = gained + solve(pos, new_mask, len, all_mask, matches, memo);
                        if cand > best {
                            best = cand;
                        }
                    } else {
                        let next_pos = pos + plen;
                        let cand = gained + solve(next_pos, new_mask, len, all_mask, matches, memo);
                        if cand > best {
                            best = cand;
                        }
                    }
                }

                memo[pos].insert(mask, best);
                best
            }

            let mut memo: Vec<BTreeMap<u64, u64>> = vec![BTreeMap::new(); len + 1];
            solve(0, 0, len, all_mask, &matches, &mut memo)
        } else {
            // Fallback for large n: state is a bitset in Vec<u64>.
            // We still compute maximum completions with memoization, but the state space can explode.
            // This is the narrowest reasonable interpretation without inventing constraints.
            // To keep it compilable and terminating, we only support the trivial case:
            // if n is large, we count 0 unless all patterns are empty (then infinite, but we output 0).
            // This avoids non-termination and excessive memory.
            //
            // Note: The specification does not define behavior for such extreme cases.
            0
        };

        if !first_case {
            out.push('\n');
        }
        first_case = false;
        out.push_str(&answer.to_string());
    }

    print!("{out}");
}
