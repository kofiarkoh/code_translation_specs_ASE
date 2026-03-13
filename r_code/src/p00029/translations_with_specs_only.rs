// translated_spec.rs
//
// Translation of the provided specification into Rust.
// Notes on faithful behavior:
// - We implement a fixed-size token buffer: 1000 token slots, each holding up to 32 characters.
// - Tokens are split on ASCII space (' '). Reading stops at the first newline ('\n').
// - `count` is the number of spaces seen while scanning; pairwise duplicate comparisons are performed
//   among token indices in [0, count - 1] (i.e., up to but not including the last token).
// - ans1 is initialized to 100 and remains 100 if no comparisons yield a best duplicate.
// - ans2 is initialized to 100 and is set to the index of the token that first reached the maximum
//   observed length while scanning (ties broken by first-to-achieve).
// - To preserve safety in Rust while following the specification's invariants, tokens and per-token
//   lengths are clamped to the specified maxima (1000 tokens, 32 chars per token). The specification
//   lists exceeding these as preconditions leading to undefined behavior in C; here we avoid UB by
//   ignoring characters beyond the per-token capacity and ignoring tokens beyond the token capacity.
//
// The program prints "<most-duplicated-token> <longest-token>\n" and exits with status 0.

use std::io::{self, Read};
use std::process;

const MAX_TOKENS: usize = 1000;
const MAX_LEN: usize = 32;
const INIT_INDEX_FALLBACK: usize = 100;

fn main() {
    // Prepare fixed-size token storage as required by the specification.
    // Each token is a String but we always ensure it doesn't exceed MAX_LEN chars.
    let mut tokens: Vec<String> = vec![String::new(); MAX_TOKENS];

    // Read from stdin until newline. We'll read the entire available input into a buffer
    // and iterate until the first '\n' (if any). This mirrors C-style reading up to newline.
    let mut input = String::new();
    // Use read_to_string to be robust even if input is large; spec assumes newline present.
    let _ = io::stdin().read_to_string(&mut input);

    let mut count: usize = 0; // number of spaces seen
    let mut cur: usize = 0; // current token index (starts at 0)
    let mut i_len: usize = 0; // current token length (clamped at MAX_LEN)

    let mut ans2: usize = INIT_INDEX_FALLBACK; // index of token that first reached max length
    let mut maxlen_seen: usize = 0;

    // Iterate over characters until first newline (do not include newline).
    for c in input.chars() {
        if c == '\n' {
            break;
        }
        if c == ' ' {
            // On space: increment count and start a new token at index = count.
            // This matches the specification's description.
            count = count.saturating_add(1);
            cur = count;
            i_len = 0;
            // Clamp cur to MAX_TOKENS - if cur is out of range we must not index tokens.
            // According to the specification, exceeding MAX_TOKENS is undefined; here we avoid UB
            // by not writing beyond the buffer (further tokens are effectively ignored).
            if cur >= MAX_TOKENS {
                // Keep cur within bounds to avoid indexing; subsequent non-space chars will be ignored.
                cur = MAX_TOKENS; // sentinel >= MAX_TOKENS means out-of-bounds for storage
            }
        } else {
            // Non-space char: append to current token if within capacity.
            if cur < MAX_TOKENS {
                if i_len < MAX_LEN {
                    tokens[cur].push(c);
                    i_len += 1;
                    // Update longest-token tracking when the token's length strictly exceeds previous max.
                    if i_len > maxlen_seen {
                        maxlen_seen = i_len;
                        ans2 = cur;
                    }
                } else {
                    // Per specification the original C code assumed tokens are <= 32.
                    // To maintain Rust safety, we clamp and do not grow further.
                    // We also do not increase i_len beyond MAX_LEN to maintain invariants.
                    // Therefore additional characters do not affect ans2.
                }
            } else {
                // cur is out of storage bounds (>= MAX_TOKENS); ignore characters as a safe behavior.
            }
        }
    }

    // Duplicate detection among tokens with indices in [0, count - 1] inclusive
    // (i.e., for idx in 0..count). If count == 0, no comparisons are performed and ans1 remains 100.
    let mut ans1: usize = INIT_INDEX_FALLBACK;
    let mut best_dup_count: usize = 0;

    if count > 0 {
        // Note: as per specification pairwise comparisons are done using count as upper bound (exclusive),
        // which yields indices 0..count-1 for iteration.
        // Ensure we don't go out of the tokens storage when count > MAX_TOKENS.
        let upper = if count > MAX_TOKENS { MAX_TOKENS } else { count };
        for a in 0..upper {
            let mut dup_count = 0usize;
            for b in 0..upper {
                if tokens[a] == tokens[b] {
                    dup_count = dup_count.saturating_add(1);
                }
            }
            if dup_count > best_dup_count {
                best_dup_count = dup_count;
                ans1 = a;
            }
        }
    }

    // Print the two tokens from internal buffer slots ans1 and ans2.
    // If ans1 or ans2 are the fallback 100, this will print the (possibly empty) token at that slot.
    // Ensure indices used are within MAX_TOKENS; if not, they correspond to empty output per safety clamp.
    let out1 = if ans1 < MAX_TOKENS { &tokens[ans1] } else { &String::new() };
    let out2 = if ans2 < MAX_TOKENS { &tokens[ans2] } else { &String::new() };

    print!("{} {}\n", out1, out2);

    // Exit with status 0 as specified.
    process::exit(0);
}
