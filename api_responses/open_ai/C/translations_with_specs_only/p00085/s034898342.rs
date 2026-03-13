use std::io::{self, Read};

fn main() {
    // Read all input from stdin and split into whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (behavior for IO errors is not specified in the spec).
        return;
    }

    let mut iter = input.split_whitespace();
    // Process pairs of integers until "0 0" is encountered.
    loop {
        let n_tok = match iter.next() {
            Some(t) => t,
            None => break, // EOF before terminating pair: spec treats this as undefined; we simply stop.
        };
        let m_tok = match iter.next() {
            Some(t) => t,
            None => break, // malformed input: stop.
        };

        // Parse tokens as signed integers per spec description; choose i64 to cover large m if present.
        // The specification assumes well-formed integer pairs; parse errors lead to termination here.
        let n_val: i64 = match n_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let m_val: i64 = match m_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: do not process the terminating pair.
        if n_val == 0 && m_val == 0 {
            break;
        }

        // The spec requires 1 <= n <= 1000 and m >= 1 for defined behavior.
        // We follow the spec's intended use and only simulate when n >= 1 and m >= 1.
        if n_val < 1 || m_val < 1 {
            // Behavior undefined by spec; here we stop processing further input.
            break;
        }

        // Cast to usize for indexing. Values beyond usize are unlikely in typical inputs.
        let n = n_val as usize;
        let m = m_val as usize;

        // Per spec, n is expected to be <= 1000. We do not enforce it beyond using a Vec of size n.
        let mut alive = vec![true; n];
        let mut removed = 0usize;
        let mut idx = 0usize; // current index to examine (0-based)
        let mut last_removed = 0usize;

        while removed < n {
            let mut count = 0usize;
            loop {
                if alive[idx] {
                    count += 1;
                    if count == m {
                        // remove this position
                        alive[idx] = false;
                        last_removed = idx;
                        removed += 1;
                        // start next counting at the next position after the removed one
                        idx = (idx + 1) % n;
                        break;
                    }
                }
                idx = (idx + 1) % n;
            }
        }

        // Output is 1-based index of the last eliminated position, followed by newline.
        println!("{}", last_removed + 1);
    }
}
