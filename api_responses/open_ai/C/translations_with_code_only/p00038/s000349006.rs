use std::io::{self, Read};

fn is_straight(cards: &[i32; 5]) -> bool {
    // Mirror the C isstraight function exactly:
    // Check sequentially cards[i] + 1 == cards[i+1] for i = 0..3.
    // If any gap found, check the special case 1,10,11,12,13.
    for i in 0..4 {
        if cards[i] + 1 != cards[i + 1] {
            // special-case A,10,11,12,13
            return cards[0] == 1
                && cards[1] == 10
                && cards[2] == 11
                && cards[3] == 12
                && cards[4] == 13;
        }
    }
    true
}

// Parse five ints from the input string starting at `pos`, following the scanf("%d,%d,%d,%d,%d") semantics:
// - skip leading whitespace before the first %d
// - for each %d: skip leading whitespace before parsing the integer
// - after each parsed integer except the last, require a literal comma immediately after the digits (no whitespace allowed between digits and comma)
// Returns Some(([i32;5], new_pos)) on success, or None if parsing cannot proceed (including EOF).
fn parse_five_ints(s: &str, mut pos: usize) -> Option<([i32; 5], usize)> {
    let bytes = s.as_bytes();
    let len = bytes.len();

    // helper: skip ASCII whitespace
    let mut skip_ws = |p: &mut usize| {
        while *p < len && bytes[*p].is_ascii_whitespace() {
            *p += 1;
        }
    };

    // helper: parse one signed integer starting at pos (after skipping whitespace as caller desires).
    // Returns Some((value, new_pos)) if at least one digit found, else None.
    let parse_int = |p: &mut usize| -> Option<i32> {
        // skip leading whitespace as per %d behavior
        while *p < len && bytes[*p].is_ascii_whitespace() {
            *p += 1;
        }
        if *p >= len {
            return None;
        }
        let mut sign = 1i32;
        if bytes[*p] == b'+' {
            *p += 1;
        } else if bytes[*p] == b'-' {
            sign = -1;
            *p += 1;
        }
        // At least one digit required
        if *p >= len || !bytes[*p].is_ascii_digit() {
            return None;
        }
        let mut acc: i32 = 0;
        while *p < len && bytes[*p].is_ascii_digit() {
            // accumulate with wrapping to emulate potential wraparound behavior
            let digit = (bytes[*p] - b'0') as i32;
            acc = acc.wrapping_mul(10).wrapping_add(digit);
            *p += 1;
        }
        if sign == -1 {
            acc = acc.wrapping_neg();
        }
        Some(acc)
    };

    // Before first %d, skip whitespace
    skip_ws(&mut pos);
    let mut cards = [0i32; 5];
    for k in 0..5 {
        // Parse integer (each %d itself skips leading whitespace)
        match parse_int(&mut pos) {
            Some(v) => cards[k] = v,
            None => return None,
        }
        if k < 4 {
            // After a number (except last), the format expects an immediate comma.
            // No whitespace allowed between number and comma (matching C's scanf behavior with literal comma).
            if pos >= len {
                return None;
            }
            if bytes[pos] != b',' {
                return None;
            }
            pos += 1; // consume the comma
            // After comma, the next %d will skip whitespace itself, so we don't skip here necessarily.
        }
    }
    Some((cards, pos))
}

fn main() {
    // yakuname array as in C
    let yakuname = [
        "null",
        "one pair",
        "two pair",
        "three card",
        "straight",
        "full house",
        "four card",
    ];

    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin, exit silently (no outputs), similar to C failing early.
        return;
    }

    let mut pos = 0usize;
    let len = input.len();

    // Repeatedly parse groups of five ints in the scanf("%d,%d,%d,%d,%d") manner.
    loop {
        // Skip leading whitespace before attempting to parse the next set.
        // But parse_five_ints already skips leading whitespace before first %d, so we can call it directly.
        if pos >= len {
            break;
        }
        match parse_five_ints(&input, pos) {
            Some((mut cards, new_pos)) => {
                pos = new_pos;
                // Perform the same bubble sort as the C code
                for i in (1..=4).rev() {
                    for j in 0..i {
                        if cards[j] > cards[j + 1] {
                            let temp = cards[j];
                            cards[j] = cards[j + 1];
                            cards[j + 1] = temp;
                        }
                    }
                }

                // Determine yaku following the exact C logic
                let yaku = {
                    let mut yaku = 0i32;
                    if cards[1] == cards[2] && cards[2] == cards[3] {
                        if cards[0] == cards[1] || cards[3] == cards[4] {
                            yaku = 6; // four card (three-of-a-kind plus a pair -> full house? Wait mapping: but preserve C)
                        } else {
                            yaku = 3; // three card
                        }
                    } else if cards[0] == cards[1] && cards[1] == cards[2] {
                        if cards[3] == cards[4] {
                            yaku = 5;
                        } else {
                            yaku = 3;
                        }
                    } else if cards[2] == cards[3] && cards[3] == cards[4] {
                        if cards[0] == cards[1] {
                            yaku = 5;
                        } else {
                            yaku = 3;
                        }
                    } else if is_straight(&cards) {
                        yaku = 4;
                    } else {
                        // count pairs
                        let mut cnt = 0;
                        for i in 0..4 {
                            if cards[i] == cards[i + 1] {
                                cnt += 1;
                            }
                        }
                        yaku = cnt;
                    }
                    yaku as usize
                };

                // Print the corresponding name followed by newline
                // Ensure index is within bounds; in C yaku could be 0..6, here also.
                if yaku < yakuname.len() {
                    println!("{}", yakuname[yaku]);
                } else {
                    // Defensive: if somehow out of bounds, print "null" to avoid panic.
                    println!("{}", yakuname[0]);
                }
            }
            None => {
                // If parsing fails or EOF is reached in a way that doesn't produce five ints, stop processing.
                break;
            }
        }
    }
}
