// translator.rs
//
// Translation of the provided specification into Rust.
//
// Notes:
// - The specification describes reading blocks of up to 8 lines (each 8 chars '0'/'1'),
//   collecting up to 4 non-empty rows (rows containing at least one '1'), normalizing
//   those row values by dividing by the smallest, and then classifying the block
//   into one of the letters A..G based on the number of collected rows and the
//   normalized values at specific positions.
//
// - The specification does NOT supply the exact mapping from normalized patterns
//   to letters. To produce a complete, deterministic, and compilable program that
//   preserves the observable I/O contract (one uppercase letter A..G + newline per
//   processed block) while remaining faithful to "classification depends on the
//   number of collected rows and normalized values", this implementation chooses
//   a simple deterministic mapping that inspects `h` (count of collected rows)
//   and some normalized entries to pick a letter.
//
// - This mapping is deliberately minimal and deterministic. It uses only the
//   specified inputs (h and normalized values) and thus stays within the narrowest
//   reasonable interpretation of the unspecified classification rules.
//
// - The program assumes well-formed input per the specification (lines of exactly
//   8 characters, only '0' and '1', blocks followed by a blank line). It does not
//   perform thorough validation (matching the specification's guidance on error handling).
//
// Behavior summary:
// - Repeatedly:
//   - Attempt to read the first line of a block. If EOF occurs here, exit with status 0.
//   - Read up to 8 lines for the block, stopping early if an empty (blank) line appears
//     which serves as the block separator. For each non-blank 8-character line,
//     if it contains at least one '1' it is converted to a u8 in 1..=255 and collected
//     (up to 4 collected rows).
//   - If the block terminated because we read 8 data lines (no blank separator seen yet),
//     attempt to read and discard one additional line as the separator.
//   - Normalize collected values by integer-dividing by the smallest collected value.
//   - Classify deterministically into one of 'A'..'G' using h and normalized values.
//   - Print the single-letter classification followed by a newline.
// - Then proceed to the next block.
//
// This program tries to follow the specification closely while being robust in normal cases.

use std::io::{self, BufRead};

fn parse_binary_line_to_u8(s: &str) -> u8 {
    // s is expected to be exactly 8 characters of '0'/'1'.
    // Convert MSB first: leftmost char is highest-order bit.
    let mut val: u8 = 0;
    for ch in s.chars() {
        val = (val << 1)
            | match ch {
                '0' => 0,
                '1' => 1,
                // Per specification, invalid chars are unspecified; but inputs are assumed valid.
                // We'll treat any other char as 0 to avoid panics in malformed inputs.
                _ => 0,
            };
    }
    val
}

fn classify_block(collected: &[u8]) -> char {
    // collected is non-empty and length <= 4 per specification assumptions.
    // Compute minimum and normalized integer-divides.
    let h = collected.len();
    let &minv = collected.iter().min().expect("collected must be non-empty");
    // minv in 1..=255 per spec assumptions
    let normalized: Vec<u8> = collected
        .iter()
        .map(|&v| {
            // integer divide; both are u8 but promote to u16 to avoid accidental overflow
            // though values are small; result fits in u8.
            let div = (v as u16) / (minv as u16);
            div as u8
        })
        .collect();

    // Deterministic mapping chosen because the spec omits the exact mapping.
    // The mapping uses h and normalized values at positions 1,2 as described in spec.
    //
    // Mapping (chosen minimally):
    // - h == 1 -> 'A'
    // - h == 2:
    //     - if normalized[1] == 2 -> 'B' else -> 'C'
    // - h == 3:
    //     - if normalized[1] == 2 && normalized[2] == 3 -> 'D' else -> 'E'
    // - h == 4 -> 'F'
    // - otherwise -> 'G'  (fallback; shouldn't happen under assumed constraints)
    match h {
        1 => 'A',
        2 => {
            if normalized.get(1).copied().unwrap_or(0) == 2 {
                'B'
            } else {
                'C'
            }
        }
        3 => {
            if normalized.get(1).copied().unwrap_or(0) == 2
                && normalized.get(2).copied().unwrap_or(0) == 3
            {
                'D'
            } else {
                'E'
            }
        }
        4 => 'F',
        _ => 'G',
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Outer loop over blocks.
    loop {
        // Attempt to read the first line of a block.
        let first_line_opt = lines.next();
        let first_line = match first_line_opt {
            Some(Ok(s)) => s,
            Some(Err(_)) => {
                // I/O error while reading: per spec unspecified; exit non-zero.
                // We choose to exit with code 1.
                std::process::exit(1);
            }
            None => {
                // EOF when attempting to read the first line of a new block:
                // clean termination with exit status 0.
                return;
            }
        };

        // Remove only trailing CR/LF characters for blank detection and length checks.
        let mut trimmed = first_line.trim_end_matches('\n').trim_end_matches('\r').to_string();

        // If the first line is the separator (blank), that violates the assumed preconditions.
        // The spec assumes the first line of a block is a data line. Behavior is unspecified.
        // We'll treat a blank first line as an empty block separator and continue to next block.
        if trimmed.is_empty() {
            // Skip and continue to next block (attempt to read a new first line).
            continue;
        }

        // Collect up to 8 data lines for this block, starting with the first_line we already read.
        let mut data_lines: Vec<String> = Vec::new();
        data_lines.push(trimmed);

        // read up to 7 more lines (total up to 8). Stop early if we encounter a blank line
        // which serves as block separator (we consume it).
        let mut saw_separator = false;
        for _ in 0..7 {
            match lines.next() {
                Some(Ok(s)) => {
                    let s_trimmed = s.trim_end_matches('\n').trim_end_matches('\r').to_string();
                    if s_trimmed.is_empty() {
                        // Separator found; block ended; we consumed the separator already.
                        saw_separator = true;
                        break;
                    } else {
                        data_lines.push(s_trimmed);
                    }
                }
                Some(Err(_)) => {
                    // I/O error while reading: unspecified per spec. Exit non-zero.
                    std::process::exit(1);
                }
                None => {
                    // EOF encountered while reading block lines. Spec does not define exact behavior.
                    // We'll treat this as an end-of-input with an incomplete block; to be conservative
                    // and avoid producing output for an incomplete block, exit normally.
                    return;
                }
            }
        }

        // If we read 8 data lines and did not see a separator, the spec says the program
        // reads and discards one additional line as the block separator. Attempt to read it.
        if !saw_separator && data_lines.len() == 8 {
            match lines.next() {
                Some(Ok(_s)) => {
                    // Discard separator line regardless of content (spec expects blank).
                    // Do nothing.
                }
                Some(Err(_)) => std::process::exit(1),
                None => {
                    // EOF while attempting to read separator: unspecified. We'll exit cleanly.
                    return;
                }
            }
        }

        // Now process the collected data_lines for this block.
        // For each line that contains at least one '1', convert to integer and collect.
        let mut collected: Vec<u8> = Vec::new();
        for dl in &data_lines {
            // As per spec, lines are expected to be exactly 8 characters of '0'/'1'.
            // We do not validate; assume valid input.
            if dl.contains('1') {
                let v = parse_binary_line_to_u8(&dl);
                if v != 0 {
                    // Store up to 4 collected rows (the original program allocated storage for up to 4).
                    if collected.len() < 4 {
                        collected.push(v);
                    } else {
                        // If more than 4 non-zero rows appear, behavior is unspecified by the spec.
                        // Here we ignore additional non-zero rows (narrow, safe choice).
                    }
                }
            }
        }

        // According to the specification, at least one non-zero row must exist in the block.
        // If collected is empty, input violated preconditions; behavior unspecified. We'll
        // simply skip producing output for this block (or we could choose a default). We'll
        // choose to output 'G' as a fallback to ensure one-letter-per-block for well-formed blocks only.
        if collected.is_empty() {
            // Fallback letter for malformed/unspecified cases:
            println!("G");
            continue;
        }

        // Normalize and classify.
        let letter = classify_block(&collected);
        println!("{}", letter);
    }
}
