// main.rs
//
// Translation of the given specification into Rust.
// Behavior adheres to the Specification described in the prompt.
// Uses i32 to represent C `int` semantics described in the spec.
// Wrapping arithmetic is used for increments/decrements to avoid Rust overflow panics
// and to better emulate low-level wrapping behavior where loops rely on ++/-- semantics.

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string and parse whitespace-separated tokens as i32.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, mimic the unspecified/undefined behavior of the C program:
        // just exit with success without producing output.
        return;
    }

    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>());

    // Read n (first integer). If missing or parse fails, behave like the C program would
    // on failed scanf: produce no output and exit.
    let n_i32 = match iter.next().and_then(|r| r.ok()) {
        Some(v) => v,
        None => return,
    };

    // In C, a negative n would result in zero iterations of the typical for loop.
    // Emulate that by treating negative n as zero pairs.
    let pair_count = if n_i32 > 0 {
        // If n is positive, convert to usize for loop counting. If it's absurdly large and doesn't fit,
        // clamp to usize::MAX to avoid panic (this is an extreme corner; spec allows undefined behavior).
        match usize::try_from(n_i32) {
            Ok(v) => v,
            Err(_) => usize::MAX,
        }
    } else {
        0usize
    };

    // Collect remaining tokens into a Vec<i32> as pairs. If there are fewer than expected,
    // behavior per spec is undefined; here we simply stop when data runs out.
    let mut nums: Vec<i32> = Vec::with_capacity(pair_count * 2);
    for _ in 0..(pair_count * 2) {
        match iter.next() {
            Some(Ok(v)) => nums.push(v),
            _ => break,
        }
    }

    // If we didn't obtain at least pair_count * 2 integers, mimic undefined behavior by
    // processing as many complete pairs as we have.
    let available_pairs = nums.len() / 2;
    let to_process = std::cmp::min(pair_count, available_pairs);

    let mut out_lines: Vec<String> = Vec::with_capacity(to_process);

    for i in 0..to_process {
        let from = nums[2 * i];
        let to = nums[2 * i + 1];

        // According to spec, rules are evaluated in order:
        // 1) if from < to: inclusive ascending from..to
        // 2) else if to == 5: ascending from..9 (those j <= 9), then single token 5
        // 3) else if from > to:
        //    a) if from in 6..=9: ascending from..9, then descending 5..(to+1) as applicable, then token to
        //    b) else: inclusive descending from..to
        // Special case: if from == to and to != 5 -> produce no output line for that pair.

        if from < to {
            // inclusive ascending from..to
            let mut tokens: Vec<String> = Vec::new();
            let mut j = from;
            // Use wrapping_add to avoid Rust panic on overflow and better emulate C increment semantics.
            loop {
                tokens.push(j.to_string());
                if j == to {
                    break;
                }
                j = j.wrapping_add(1);
                // To avoid infinite loops in pathological wrapping cases (which are undefined in C),
                // we rely on the condition above to break when j == to.
            }
            out_lines.push(tokens.join(" "));
        } else if to == 5 {
            // ascending from..9 for j <= 9, then the single token 5
            let mut tokens: Vec<String> = Vec::new();
            let mut j = from;
            // Add ascending part only for j <= 9
            while j <= 9 {
                tokens.push(j.to_string());
                if j == 9 {
                    break;
                }
                j = j.wrapping_add(1);
            }
            // Append single token 5 (always present for this rule)
            tokens.push("5".to_string());
            out_lines.push(tokens.join(" "));
        } else if from > to {
            if (6..=9).contains(&from) {
                // ascending from..9 (those j <= 9), then descending 5..(to+1) as applicable, then token to
                let mut tokens: Vec<String> = Vec::new();
                let mut j = from;
                while j <= 9 {
                    tokens.push(j.to_string());
                    if j == 9 {
                        break;
                    }
                    j = j.wrapping_add(1);
                }
                // descending from 5 down to (to+1) inclusive, only if to < 5
                if to < 5 {
                    let mut k = 5;
                    // descend while k >= to+1
                    // compute to+1 with wrapping to mirror C-like arithmetic for extreme cases
                    let to_plus_one = to.wrapping_add(1);
                    loop {
                        // we want to include values k where k >= to_plus_one (signed compare)
                        if k < to_plus_one {
                            break;
                        }
                        tokens.push(k.to_string());
                        if k == to_plus_one {
                            break;
                        }
                        k = k.wrapping_sub(1);
                    }
                }
                // finally append token to
                tokens.push(to.to_string());
                out_lines.push(tokens.join(" "));
            } else {
                // inclusive descending from..to
                let mut tokens: Vec<String> = Vec::new();
                let mut j = from;
                loop {
                    tokens.push(j.to_string());
                    if j == to {
                        break;
                    }
                    j = j.wrapping_sub(1);
                }
                out_lines.push(tokens.join(" "));
            }
        } else {
            // from == to
            // Special-case: if from == to and to != 5, produce no output for that pair.
            // If to == 5, it would have matched earlier branch (to == 5), so here to != 5.
            // So we do nothing (no line appended).
        }
    }

    // Print all lines. Each produced line must be terminated by exactly one newline.
    let mut stdout = String::new();
    for line in out_lines {
        stdout.push_str(&line);
        stdout.push('\n');
    }

    // Write to actual stdout. If writing fails, we cannot do much per spec; just return.
    let _ = std::io::stdout().write_all(stdout.as_bytes());
}
