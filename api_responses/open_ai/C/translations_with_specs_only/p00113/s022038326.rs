// long_division.rs
//
// Reads lines containing two non-negative integers `a b` (with 0 <= a < b).
// For each pair, performs digit-by-digit long division of a by b to produce
// the fractional digits. If the fractional expansion terminates, prints the
// digits and a newline. If it repeats, prints the digits up to the first
// detection of the repeat, then a second line with spaces and '^' characters
// indicating the start and length of the repeating cycle.
//
// This implementation follows the specification exactly:
// - It maps each non-zero remainder to the index of the digit that will be
//   produced next when that remainder is encountered (i.e., the 0-based index
//   of the fractional digit about to be generated).
// - It sets rem_pos[remainder] = idx before multiplying by 10 and producing
//   the next digit. When a remainder recurs, the repeating cycle starts at
//   rem_pos[remainder], and its length is idx - rem_pos[remainder].
// - The digit that leads to producing a remainder that is seen again is
//   included in the printed digits (matches examples such as 1/3 -> "3\n" and
//   "^\n").
//
// Notes about input handling:
// - The spec assumes each non-empty input line contains exactly two integers
//   separated by a single ASCII space and terminated by newline. We attempt
//   to parse lines in that form; malformed lines are ignored (behavior for
//   malformed input is unspecified by the spec).
// - A small safety check enforces the internal MAX bound described in the
//   specification. Lines violating preconditions (b == 0, a >= b, b too
//   large, parse errors) are skipped.

use std::io::{self, BufRead};

const MAX: usize = 1_000_000; // as described in the specification

fn main() {
    let stdin = io::stdin();
    let mut out = io::stdout();
    let lock = stdin.lock();

    for line_res in lock.lines() {
        // If reading a line fails, stop processing further.
        let line = match line_res {
            Ok(s) => s,
            Err(_) => break,
        };

        // Skip empty lines (spec allows zero or more lines; non-empty lines contain two ints)
        if line.is_empty() {
            continue;
        }

        // Expect exactly two tokens separated by a single ASCII space.
        // We tolerate exactly one ' ' separator and no other tokens; this stays close
        // to the spec's stated input format.
        let mut parts = line.splitn(3, ' ');
        let a_str = parts.next();
        let b_str = parts.next();
        let extra = parts.next(); // if Some, there were more than 2 parts

        if a_str.is_none() || b_str.is_none() || extra.is_some() {
            // Malformed line per the narrow interpretation; skip it.
            continue;
        }
        let a_str = a_str.unwrap();
        let b_str = b_str.unwrap();

        // Parse as usize (non-negative integers). If parsing fails, skip the line.
        let a: usize = match a_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let b: usize = match b_str.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Enforce preconditions: b >= 1, a < b, and b not excessively large.
        if b == 0 || a >= b || b > (MAX + 1) {
            // Spec says behavior is undefined for these; we choose to skip gracefully.
            continue;
        }

        // rem_pos[r] == -1 means remainder r has not been seen yet.
        // Size is b because remainders are in 0..b-1. We do not use index 0 except to
        // check for immediate termination (remainder == 0 stops the process).
        let mut rem_pos = vec![-1i32; b];

        let mut digits = String::new();
        let mut remainder = a;
        let mut idx: i32 = 0;

        // Loop invariant:
        // - rem_pos[r] stores the 0-based index of the digit that will be produced
        //   next when remainder r is encountered.
        // - We store rem_pos[remainder] = idx before generating the next digit.
        while remainder != 0 && rem_pos[remainder] == -1 {
            rem_pos[remainder] = idx;
            let r10 = remainder * 10;
            let digit = r10 / b; // 0..9
            // Append digit character
            digits.push((b'0' + (digit as u8)) as char);
            remainder = r10 % b;
            idx += 1;
        }

        if remainder == 0 {
            // terminating decimal: print digits and newline
            // (digits may be empty, e.g., a == 0 -> print an empty line)
            println!("{}", digits);
        } else {
            // repeating decimal:
            // rem_pos[remainder] is the start index (0-based) of the cycle
            let start = rem_pos[remainder] as usize;
            let len = (idx as usize) - start;

            // First line: digits up to the moment of detection (we already produced them)
            println!("{}", digits);

            // Second line: spaces (start) followed by '^' repeated len times
            // (If start or len are zero, this still prints the correct alignment.)
            let mut align = String::new();
            align.extend(std::iter::repeat(' ').take(start));
            align.extend(std::iter::repeat('^').take(len));
            println!("{}", align);
        }

        // Flush after each processed line to preserve observable I/O ordering.
        let _ = out.flush();
    }
}
