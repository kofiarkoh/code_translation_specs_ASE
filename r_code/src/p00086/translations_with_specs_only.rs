// Eulerian graph checker translated from the given specification.
// Reads whitespace-separated integer pairs from stdin. Each dataset is a sequence of pairs
// terminated by the sentinel pair "0 0". For each completed dataset, prints "OK" if the
// graph (undirected) described by the pairs has 0 or 2 vertices of odd degree, otherwise "NG".
//
// Translation notes (kept faithful to the Specification):
// - Vertex indices are used as array indices directly. The array has 101 entries (0..100).
//   This preserves the behavior that index 0 is addressable (as many original implementations),
//   while indices outside 0..100 will panic at runtime (approximating undefined/out-of-bounds behavior).
// - A pair (a,b) where (a == 0 && b == 0) ends the current dataset. Any other pair is treated
//   as an edge (including when one coordinate is 0).
// - When evaluating a dataset, the code inspects degrees starting at vertex 1 and stops at the
//   first vertex index with degree == 0. Only that contiguous prefix is considered when counting
//   odd-degree vertices (as described in the Specification).
// - If input ends or a non-integer token is encountered such that a full pair cannot be read,
//   the program exits without producing a result for the trailing incomplete dataset.
// - Output lines are exactly "OK\n" or "NG\n" per completed dataset.

use std::io::{self, Read};

fn main() {
    // Read whole stdin into a string and split by whitespace to tokens.
    // Any non-integer token will cause parsing to fail and program to terminate
    // without output for the incomplete dataset (matching the spec).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (spec: process returns 0 on normal termination).
        return;
    }

    let mut tokens = input.split_whitespace();

    // degrees[0..=100]; indices outside this range will cause a runtime panic on indexing,
    // approximating the undefined/out-of-bounds behavior described in the Specification.
    let mut degrees = [0i32; 101];

    loop {
        // Attempt to read first integer of pair
        let s_a = match tokens.next() {
            Some(s) => s,
            None => break, // EOF before a new pair: terminate quietly
        };
        let a: i32 = match s_a.parse() {
            Ok(v) => v,
            Err(_) => break, // Non-integer token: stop reading and terminate (no output for incomplete dataset)
        };

        // Attempt to read second integer of pair
        let s_b = match tokens.next() {
            Some(s) => s,
            None => break, // EOF while reading a pair: incomplete dataset -> terminate without printing
        };
        let b: i32 = match s_b.parse() {
            Ok(v) => v,
            Err(_) => break, // Non-integer token: terminate
        };

        // Sentinel check: (0,0) ends current dataset
        if a == 0 && b == 0 {
            // Evaluate dataset: inspect vertices 1..=100, but stop at first degree == 0
            let mut odd = 0usize;
            for i in 1..=100 {
                let d = degrees[i];
                if d == 0 {
                    break;
                }
                if d % 2 != 0 {
                    odd += 1;
                }
            }
            if odd == 0 || odd == 2 {
                println!("OK");
            } else {
                println!("NG");
            }

            // Reset degrees for next dataset
            degrees = [0i32; 101];
        } else {
            // Treat (a,b) as an undirected edge; increment degrees for both endpoints.
            // This uses direct indexing; if a or b are outside 0..=100 (and not both zero),
            // this will panic (approximating undefined behavior per the Specification).
            let ai = a as usize;
            let bi = b as usize;
            degrees[ai] = degrees[ai].wrapping_add(1); // use wrapping_add to mirror possible overflow behavior
            degrees[bi] = degrees[bi].wrapping_add(1);
        }
    }

    // Normal termination; implicit exit status 0.
}
