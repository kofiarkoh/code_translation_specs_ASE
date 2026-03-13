// translated from the provided C implementation, following the SPEC.
// Computes the number of distinct n-element subsets of {0..9} whose sum equals s.
// Reads pairs of integers (n s) from stdin, prints one line per pair (except the terminating 0 0 pair).
// Behavior on malformed input is intentionally minimal (stop processing) to mirror the unspecified behavior in the spec/C code.

use std::io::{self, Read};

fn get_way(start: usize, n: i32, sum: i32, target: i32, used: &mut [bool; 10]) -> i32 {
    // If we've selected the required count, check whether the running sum equals the target.
    if n == 0 {
        return if sum == target { 1 } else { 0 };
    }

    let mut ret: i32 = 0;
    // Iterate digits from `start` to 9 inclusive.
    // Note: we pass `i` as the next `start` to the recursive call (same as the C code).
    // This, combined with marking used[i]=true, ensures digits are not reused and selections are order-insensitive.
    for i in start..10 {
        let digit = i as i32;
        if !used[i] && sum + digit <= target {
            used[i] = true;
            ret += get_way(i, n - 1, sum + digit, target, used);
            used[i] = false;
        }
    }
    ret
}

fn main() {
    // Read entire stdin and parse whitespace-separated tokens as integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit (spec leaves malformed input behavior unspecified).
        return;
    }

    let mut tokens = input.split_whitespace();
    loop {
        // Read next pair of integers. If missing or unparsable, stop processing.
        let n_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let s_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        let n: i32 = match n_tok.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed -> stop (mirrors unspecified behavior)
        };
        let s: i32 = match s_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination sentinel: do not print a result for (0, 0).
        if n == 0 && s == 0 {
            break;
        }

        // Reset used digits for each query.
        let mut used = [false; 10];
        let ways = get_way(0, n, 0, s, &mut used);
        println!("{}", ways);
    }
}
