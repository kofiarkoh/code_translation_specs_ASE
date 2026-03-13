// poker_hand_classifier.rs
//
// Translated from the provided C code, following the SPEC as the source of truth.
// Behavior:
// - Reads zero or more records from stdin, each record formatted as five integers
//   separated by literal commas: "%d,%d,%d,%d,%d".
// - For each successfully parsed record, classifies the 5-card hand and prints
//   one of the exact strings:
//     "null", "one pair", "two pair", "three card", "straight", "full house", "four card"
//   each followed by a newline.
//
// Notes about parsing and differences vs the C original:
// - The SPEC allows malformed-input behavior to be undefined. This Rust
//   implementation requires exact matching of the pattern described: before
//   each %d any amount of whitespace is allowed (like scanf), the integer
//   itself is parsed (optionally signed), and for the first four integers a
//   comma must immediately follow the digits (no intervening whitespace).
//   This matches scanf semantics for "%d," where %d consumes leading
//   whitespace but the literal comma must match the next character.
// - On EOF before a new record begins (no more integers), the program exits
//   normally (exit code 0). If a record cannot be parsed because the input
//   doesn't match the required pattern, parsing fails and processing stops.
//   (The C program's behavior in malformed cases is unspecified; stopping is a
//   safe choice consistent with the SPEC's allowance of undefined behavior.)
// - All classification logic follows the SPEC and the original C logic.

use std::io::{self, Read};

fn is_straight(cards: &[i32; 5]) -> bool {
    // Assumes cards are sorted ascending.
    // Straight if consecutive differences of 1, or the special Ace-high: [1,10,11,12,13].
    let mut consecutive = true;
    for i in 0..4 {
        if cards[i] + 1 != cards[i + 1] {
            consecutive = false;
            break;
        }
    }
    if consecutive {
        return true;
    }
    // check special ace-high straight
    cards[0] == 1 && cards[1] == 10 && cards[2] == 11 && cards[3] == 12 && cards[4] == 13
}

/// Attempt to parse one record of five integers following the scanf-style pattern:
/// "%d,%d,%d,%d,%d"
/// - `s` is the full input string
/// - `pos` is the current byte index into `s` and will be advanced past the record on success
/// Returns Some([i32;5]) on success, or None if EOF/malformed / no more complete record.
fn parse_record(s: &str, pos: &mut usize) -> Option<[i32; 5]> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    let mut vals: [i32; 5] = [0; 5];

    // Helper to skip ASCII whitespace
    let mut skip_ws = |p: &mut usize| {
        while *p < len && bytes[*p].is_ascii_whitespace() {
            *p += 1;
        }
    };

    for field in 0..5 {
        // For each %d: skip leading whitespace
        skip_ws(pos);

        if *pos >= len {
            // EOF before next integer -> no more complete records
            return None;
        }

        // Parse optional sign
        let mut negative = false;
        if bytes[*pos] == b'+' || bytes[*pos] == b'-' {
            negative = bytes[*pos] == b'-';
            *pos += 1;
            // if sign present but no digit after -> malformed
            if *pos >= len || !bytes[*pos].is_ascii_digit() {
                return None;
            }
        } else if !bytes[*pos].is_ascii_digit() {
            // Next non-whitespace char must begin digits for %d
            return None;
        }

        // Parse digits (at least one digit guaranteed here)
        let mut acc: i64 = 0;
        let mut any_digit = false;
        while *pos < len && bytes[*pos].is_ascii_digit() {
            any_digit = true;
            acc = acc * 10 + (bytes[*pos] - b'0') as i64;
            *pos += 1;
            // We parse into i64 to avoid intermediate overflow; final cast to i32
        }
        if !any_digit {
            return None;
        }
        if negative {
            acc = -acc;
        }
        // Cast to i32; mirrors C int behavior enough for this utility's domain
        vals[field] = acc as i32;

        // For the first four fields, require an immediate comma character (no whitespace allowed)
        if field < 4 {
            if *pos >= len {
                // Missing comma / EOF -> malformed/incomplete
                return None;
            }
            if bytes[*pos] != b',' {
                // Literal comma must match exactly
                return None;
            }
            // consume the comma and continue; the next %d will skip whitespace before digits
            *pos += 1;
        }
    }

    Some(vals)
}

fn classify_and_print(mut cards: [i32; 5]) {
    // sort ascending
    cards.sort();

    let yakuname = [
        "null",
        "one pair",
        "two pair",
        "three card",
        "straight",
        "full house",
        "four card",
    ];

    let mut yaku = 0usize;

    // Follow the exact decision logic from the C code to preserve precedence and edge cases
    if cards[1] == cards[2] && cards[2] == cards[3] {
        if cards[0] == cards[1] || cards[3] == cards[4] {
            yaku = 6; // four card
        } else {
            yaku = 3; // three card
        }
    } else if cards[0] == cards[1] && cards[1] == cards[2] {
        if cards[3] == cards[4] {
            yaku = 5; // full house
        } else {
            yaku = 3; // three card
        }
    } else if cards[2] == cards[3] && cards[3] == cards[4] {
        if cards[0] == cards[1] {
            yaku = 5; // full house
        } else {
            yaku = 3; // three card
        }
    } else if is_straight(&cards) {
        yaku = 4; // straight
    } else {
        // count adjacent equal pairs for pair/two pair/one pair/null
        let mut cnt = 0usize;
        for i in 0..4 {
            if cards[i] == cards[i + 1] {
                cnt += 1;
            }
        }
        yaku = cnt;
        // cnt mapping: 0 -> null (0), 1 -> one pair (1), 2 -> two pair (2)
    }

    println!("{}", yakuname[yaku]);
}

fn main() {
    // Read entire stdin into a string
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        // If reading stdin fails (rare), exit silently (mimic typical C behavior of failing)
        // (Spec requires reading from stdin; errors are not specified.)
        eprintln!("Error reading stdin: {}", e);
        return;
    }

    let mut pos = 0usize;
    while let Some(cards) = parse_record(&input, &mut pos) {
        classify_and_print(cards);
        // continue to next record; parse_record will skip leading whitespace before next %d
    }
    // On EOF or parse failure we exit normally with code 0.
}
