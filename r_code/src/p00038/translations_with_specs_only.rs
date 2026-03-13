// poker_classify.rs
//
// Translation of the provided specification into Rust.
// The program reads zero or more records from stdin. Each record must match the
// exact pattern "%d,%d,%d,%d,%d" (C-style): five integers separated by literal commas.
// Leading whitespace before each integer is allowed (matching scanf's %d behavior),
// but whitespace between an integer and the following comma is NOT allowed (scanf
// with literal comma would fail in that case).
//
// On EOF when attempting to read the next record the program exits with code 0.
// If a record cannot be parsed as five integers separated by commas (i.e. malformed),
// the specification declares behavior unspecified; here we choose the narrowest
// reasonable behavior and stop processing further input (exit normally). Only
// successfully parsed records are classified and printed.
//
// Classification follows the exact category rules in the specification and prints
// one of the seven exact strings followed by a newline.

use std::io::{self, Read};

fn is_whitespace(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0C /* formfeed */ | 0x0B /* vertical tab */)
}

fn is_digit(b: u8) -> bool {
    b'0' <= b && b <= b'9'
}

/// Attempt to parse one signed integer starting at pos, allowing leading whitespace.
/// Returns:
/// - Ok((value, new_pos)) on success (value is i32 via wrapping cast from i64 to emulate
///   simple C-like behavior for large numbers),
/// - Err(ParseErr::Eof) if we reached EOF while skipping whitespace (no input for a next record),
/// - Err(ParseErr::Malformed) if input is present but does not form a valid integer at pos.
#[derive(Debug)]
enum ParseErr {
    Eof,
    Malformed,
}

fn parse_int(buf: &[u8], mut pos: usize) -> Result<(i32, usize), ParseErr> {
    let len = buf.len();
    // Skip leading whitespace (scanf %d skips leading whitespace)
    while pos < len && is_whitespace(buf[pos]) {
        pos += 1;
    }
    if pos >= len {
        return Err(ParseErr::Eof);
    }

    // Optional sign
    let mut neg = false;
    if buf[pos] == b'+' {
        pos += 1;
    } else if buf[pos] == b'-' {
        neg = true;
        pos += 1;
    }

    if pos >= len || !is_digit(buf[pos]) {
        // There's input but not a valid integer here
        return Err(ParseErr::Malformed);
    }

    let mut acc: i64 = 0;
    while pos < len && is_digit(buf[pos]) {
        // accumulate as i64; no overflow checking, will wrap later when cast to i32
        acc = acc * 10 + (buf[pos] as i64 - b'0' as i64);
        pos += 1;
    }

    if neg {
        acc = -acc;
    }

    // Cast to i32 using as (wraps on overflow) — a narrow reasonable choice.
    Ok((acc as i32, pos))
}

/// Attempt to parse exactly five integers separated by literal commas from buf starting at pos.
/// Returns:
/// - Ok((array_of_5_ints, new_pos)) on success,
/// - Err(ParseErr::Eof) if EOF encountered while attempting to read the next record (no more records),
/// - Err(ParseErr::Malformed) if input is present but does not match the exact "%d,%d,%d,%d,%d" pattern.
fn parse_five(buf: &[u8], mut pos: usize) -> Result<([i32; 5], usize), ParseErr> {
    let len = buf.len();
    let mut out = [0i32; 5];

    for i in 0..5 {
        let (val, new_pos) = parse_int(buf, pos)?;
        pos = new_pos;
        out[i] = val;

        if i < 4 {
            // After first four numbers, require a literal comma immediately next.
            if pos >= len {
                // Input ended unexpectedly after a number -> malformed (not EOF in the sense of "nothing to read")
                return Err(ParseErr::Malformed);
            }
            if buf[pos] != b',' {
                return Err(ParseErr::Malformed);
            }
            pos += 1; // consume comma and continue to next integer
        }
    }

    Ok((out, pos))
}

fn classify(mut cards: [i32; 5]) -> &'static str {
    // Sort ascending
    cards.sort();

    // Count multiplicities (counts of equal ranks), in the order of sorted cards
    let mut counts: Vec<usize> = Vec::new();
    let mut i = 0;
    while i < 5 {
        let mut j = i + 1;
        while j < 5 && cards[j] == cards[i] {
            j += 1;
        }
        counts.push(j - i);
        i = j;
    }
    // Sort counts descending for pattern matching
    counts.sort_unstable_by(|a, b| b.cmp(a));

    // four card: at least four equal
    if counts[0] >= 4 {
        return "four card";
    }

    // full house: counts == [3,2]
    if counts.len() == 2 && counts[0] == 3 && counts[1] == 2 {
        return "full house";
    }

    // three card: counts == [3,1,1]
    if counts.len() == 3 && counts[0] == 3 && counts[1] == 1 && counts[2] == 1 {
        return "three card";
    }

    // two pair: counts == [2,2,1]
    if counts.len() == 3 && counts[0] == 2 && counts[1] == 2 && counts[2] == 1 {
        return "two pair";
    }

    // one pair: counts == [2,1,1,1]
    if counts.len() == 4 && counts[0] == 2 && counts[1] == 1 && counts[2] == 1 && counts[3] == 1 {
        return "one pair";
    }

    // straight: five distinct cards and either consecutive or the special [1,10,11,12,13]
    let all_distinct = counts.len() == 5;
    if all_distinct {
        // check consecutive
        let mut consecutive = true;
        for k in 1..5 {
            if cards[k] != cards[k - 1] + 1 {
                consecutive = false;
                break;
            }
        }
        if consecutive {
            return "straight";
        }
        // check special Ace-high sequence [1,10,11,12,13]
        if cards == [1, 10, 11, 12, 13] {
            return "straight";
        }
    }

    // otherwise null
    "null"
}

fn main() {
    // Read entire stdin into buffer to allow parsing across arbitrary separators/newlines.
    let mut input = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut input) {
        // If reading stdin errors, exit. The specification expects normal environment; treat this as termination.
        eprintln!("Failed to read stdin: {}", e);
        return;
    }

    let mut pos: usize = 0;
    let len = input.len();

    loop {
        // If at EOF (no bytes remaining or only whitespace until EOF), stop normally.
        // We attempt to parse a record; parse_five will return Err(ParseErr::Eof) if there's
        // nothing more to read (after skipping whitespace).
        match parse_five(&input, pos) {
            Ok((mut cards, new_pos)) => {
                // parse_five returns the numbers in the order parsed, but the specification
                // requires sorting ascending before classification; classification function sorts.
                let category = classify(cards);
                println!("{}", category);
                pos = new_pos;
                // continue loop until EOF
            }
            Err(ParseErr::Eof) => {
                // EOF encountered when attempting to read a new record: exit normally with code 0.
                break;
            }
            Err(ParseErr::Malformed) => {
                // The specification declares malformed input behavior unspecified.
                // We choose the narrowest reasonable action: stop processing further records
                // and exit normally. (This avoids attempting to emulate undefined behavior.)
                break;
            }
        }

        // If we've consumed all input bytes, stop.
        // Note: it's possible that after parsing a record pos == len, so next parse will
        // hit EOF and return Err(ParseErr::Eof) and break; this check allows an early exit.
        if pos >= len {
            break;
        }
    }
}
