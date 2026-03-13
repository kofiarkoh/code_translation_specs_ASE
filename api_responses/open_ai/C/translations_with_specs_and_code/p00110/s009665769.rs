use std::io::{self, Read};

/// Reads whitespace-separated tokens from stdin. Each token is expected to be of the form
/// "<left>+<right>=<result>" where operands contain only '0'..'9' and/or 'X'.
///
/// For each token, tries digits 0..9 (ascending). For a candidate digit d, every 'X'
/// in all three operands is treated as digit d. Each operand is interpreted as a base-10
/// integer parsed left-to-right (value = value*10 + digit). If left + right == result
/// for some candidate, prints that single digit and proceeds to the next token. If none
/// match, prints "NA".
///
/// Implementation notes:
/// - Uses checked integer arithmetic (i128) when building values. If any overflow or
///   invalid character occurs while evaluating an operand for a candidate digit, that
///   candidate is considered not to match. The SPEC allows undefined behavior on overflow;
///   here we avoid panics and treat overflow as a non-match instead of wrapping (this is
///   a safer behavior).
fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for token in input.split_whitespace() {
        // Find '+' and '=' positions as per spec assumptions.
        // If missing or malformed, treat as undefined per spec; here we print "NA".
        let plus_pos = token.find('+');
        let eq_pos = token.find('=');

        let (left, right, res) = match (plus_pos, eq_pos) {
            (Some(p), Some(e)) if p < e => {
                let l = &token[..p];
                let r = &token[p + 1..e];
                let ans = &token[e + 1..];
                (l, r, ans)
            }
            _ => {
                // Malformed token; SPEC says behavior undefined. We choose to print NA.
                println!("NA");
                continue;
            }
        };

        // Helper to compute numeric value of an operand with 'X' replaced by digit d.
        // Returns None if invalid character encountered or if checked arithmetic overflows.
        fn eval_with_digit(s: &str, d: u8) -> Option<i128> {
            let mut val: i128 = 0;
            for ch in s.bytes() {
                let digit = match ch {
                    b'X' => d as i128,
                    b'0'..=b'9' => (ch - b'0') as i128,
                    // Any other character -> invalid input (SPEC: undefined); treat as invalid.
                    _ => return None,
                };
                val = val.checked_mul(10)?;
                val = val.checked_add(digit)?;
            }
            Some(val)
        }

        let mut found = false;
        for cand in 0u8..=9 {
            let lv = eval_with_digit(left, cand);
            let rv = eval_with_digit(right, cand);
            let av = eval_with_digit(res, cand);

            match (lv, rv, av) {
                (Some(lv), Some(rv), Some(av)) if lv + rv == av => {
                    println!("{}", cand);
                    found = true;
                    break;
                }
                _ => continue,
            }
        }

        if !found {
            println!("NA");
        }
    }
}
