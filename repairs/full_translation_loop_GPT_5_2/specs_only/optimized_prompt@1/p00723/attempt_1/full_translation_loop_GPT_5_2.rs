use std::io::{self, Read};

const MAX_LEN: usize = 79;
const MAX_VARIATIONS: usize = 1024;

fn reverse_bytes(s: &[u8]) -> Vec<u8> {
    let mut v = s.to_vec();
    v.reverse();
    v
}

fn swap_adjacent(s: &[u8], i: usize) -> Vec<u8> {
    let mut v = s.to_vec();
    if i + 1 < v.len() {
        v.swap(i, i + 1);
    }
    v
}

fn count_unique_variations(input: &[u8]) -> usize {
    // Fixed-capacity storage for unique variations, as per spec.
    // If more than MAX_VARIATIONS are generated, we stop adding new ones
    // to preserve the fixed memory constraint; further behavior is unspecified.
    let mut uniq: Vec<Vec<u8>> = Vec::with_capacity(MAX_VARIATIONS);

    // Narrowest reasonable interpretation of "transformations, including reversals and swaps":
    // Generate variations by repeatedly applying:
    //  - reversal of the whole string
    //  - swapping any adjacent pair
    // and count unique strings reachable under these operations.
    //
    // This is implemented as a BFS over the state space, bounded by MAX_VARIATIONS.
    let mut queue: Vec<Vec<u8>> = Vec::new();

    let start = input.to_vec();
    uniq.push(start.clone());
    queue.push(start);

    let mut q_idx = 0;
    while q_idx < queue.len() {
        let cur = &queue[q_idx];
        q_idx += 1;

        // Apply reversal
        let r = reverse_bytes(cur);
        if !uniq.iter().any(|x| x == &r) {
            if uniq.len() >= MAX_VARIATIONS {
                break;
            }
            uniq.push(r.clone());
            queue.push(r);
        }

        // Apply all adjacent swaps
        for i in 0..cur.len().saturating_sub(1) {
            let s = swap_adjacent(cur, i);
            if !uniq.iter().any(|x| x == &s) {
                if uniq.len() >= MAX_VARIATIONS {
                    break;
                }
                uniq.push(s.clone());
                queue.push(s);
            }
        }

        if uniq.len() >= MAX_VARIATIONS {
            break;
        }
    }

    uniq.len()
}

fn main() {
    // Read all input as whitespace-separated tokens (integer then strings).
    // The spec does not define behavior for missing tokens; we take the narrowest
    // approach: process as many strings as are available up to the given count.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(tok) => tok.parse::<isize>().ok().filter(|&v| v >= 0).unwrap_or(0) as usize,
        None => return,
    };

    for _ in 0..n {
        let s = match it.next() {
            Some(tok) => tok.as_bytes(),
            None => break,
        };

        // Spec: max length 79; if exceeded, behavior undefined.
        // We do not attempt to "fix" it; we just proceed with the bytes as given,
        // but cap to 79 to avoid unbounded behavior in this translation.
        let bytes = if s.len() > MAX_LEN { &s[..MAX_LEN] } else { s };

        let count = count_unique_variations(bytes);
        println!("{count}");
    }
}
