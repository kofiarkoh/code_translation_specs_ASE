use std::io::{self, BufRead};

fn main() {
    // Read stdin line by line
    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        if let Ok(line) = line_res {
            // Trim only trailing newline characters; keep other content intact.
            let s = line.trim_end();
            let bytes = s.as_bytes();

            // Build initial multiset from the first 13 bytes, if available.
            // The SPEC assumes exactly 13 digit characters in positions 0..12.
            // To avoid panics on malformed input, we take up to 13 bytes.
            let mut init_counts = [0u8; 10]; // indices 0..9, we use 1..9
            let take = if bytes.len() < 13 { bytes.len() } else { 13 };
            for i in 0..take {
                // Convert ASCII digit char to numeric value by subtracting '0' (0x30).
                // This mirrors the SPEC's "characters minus '0'" behavior.
                let v = bytes[i].wrapping_sub(b'0') as usize;
                if v <= 9 {
                    init_counts[v] = init_counts[v].saturating_add(1);
                } else {
                    // Non-digit bytes will produce values outside 0..9; treat them as ignored/0.
                    // The SPEC doesn't define behavior for invalid inputs; this keeps program robust.
                }
            }

            let mut results: Vec<char> = Vec::new();

            // For each candidate digit x from 1..=9 in ascending order:
            for x in 1usize..=9usize {
                // Duplicate-excess rule: skip if x already appears at least 4 times.
                if init_counts[x] >= 4 {
                    continue;
                }

                // Build working counts for 14-symbol multiset (initial 13 plus candidate x).
                let mut counts = init_counts;
                counts[x] = counts[x].saturating_add(1);

                // Permutation buffer for 14 positions (0..13).
                let mut perm = [0u8; 14];

                // Backtracking DFS to find any permutation satisfying constraints.
                // Returns true if a valid full permutation is found.
                fn dfs(pos: usize, counts: &mut [u8; 10], perm: &mut [u8; 14]) -> bool {
                    if pos == 14 {
                        // All positions filled successfully.
                        return true;
                    }

                    // Try digits 1..9 in ascending order (order of choices does not affect correctness,
                    // only which permutation is found first).
                    for d in 1usize..=9usize {
                        if counts[d] == 0 {
                            continue;
                        }
                        // Place digit d at position pos.
                        counts[d] -= 1;
                        perm[pos] = d as u8;

                        let mut ok = true;

                        // Constraint: first two symbols are different.
                        if pos == 1 {
                            if perm[0] == perm[1] {
                                ok = false;
                            }
                        }

                        // Constraints on the four disjoint consecutive triplets:
                        // Triplets are positions [2,3,4], [5,6,7], [8,9,10], [11,12,13].
                        // When the last element of a triplet is placed (pos == 4,7,10,13),
                        // check that the triplet is neither all-equal nor a strictly descending-by-one sequence.
                        if ok && (pos == 4 || pos == 7 || pos == 10 || pos == 13) {
                            let a = perm[pos - 2];
                            let b = perm[pos - 1];
                            let c = perm[pos];
                            // all-equal?
                            if a == b && b == c {
                                ok = false;
                            }
                            // strictly descending-by-one: a == b+1 && b == c+1
                            if ok && (a == b.wrapping_add(1) && b == c.wrapping_add(1)) {
                                ok = false;
                            }
                        }

                        let found = if ok { dfs(pos + 1, counts, perm) } else { false };

                        if found {
                            return true;
                        }

                        // Backtrack
                        counts[d] += 1;
                        // perm[pos] will be overwritten on next placement; no need to clear.
                    }

                    false
                }

                if dfs(0, &mut counts, &mut perm) {
                    // Candidate x is valid; append as character.
                    results.push(char::from(b'0' + (x as u8)));
                }
            }

            // Output according to SPEC: space-separated digits in ascending order, or "0" if none.
            if results.is_empty() {
                println!("0");
            } else {
                let out = results
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("{}", out);
            }
        } else {
            // On I/O error while reading a line, stop processing further.
            break;
        }
    }
}
