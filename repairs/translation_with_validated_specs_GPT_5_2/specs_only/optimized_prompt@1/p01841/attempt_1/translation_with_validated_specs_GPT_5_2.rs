use std::io::{self, Read};

fn is_empty_subtree(s: &str) -> bool {
    // Emptiness is determined solely by substring length being exactly 2 ("()").
    s.len() == 2
}

fn strip_one_outer_parens(s: &str) -> &str {
    // Specification assumes well-formed: non-empty subtree strings are either "()" or "(<...>)".
    // This function is only called when s is not empty ("()"), so it must be "(<...>)".
    &s[1..s.len() - 1]
}

fn atoi_c_like(s: &str) -> i32 {
    // Emulate C's atoi behavior narrowly:
    // - Skip leading ASCII whitespace
    // - Optional '+' or '-'
    // - Parse consecutive decimal digits
    // - Stop at first non-digit
    // - If no digits, result is 0
    // - Overflow/underflow in C is undefined/implementation-defined; we choose wrapping i32
    //   to preserve "non-panicking" behavior and keep semantics close to typical two's-complement.
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }

    let mut sign: i32 = 1;
    if i < bytes.len() {
        if bytes[i] == b'+' {
            i += 1;
        } else if bytes[i] == b'-' {
            sign = -1;
            i += 1;
        }
    }

    let mut acc: i32 = 0;
    let mut any = false;
    while i < bytes.len() {
        let b = bytes[i];
        if !b.is_ascii_digit() {
            break;
        }
        any = true;
        let d = (b - b'0') as i32;
        acc = acc.wrapping_mul(10).wrapping_add(d);
        i += 1;
    }

    if !any {
        0
    } else if sign == 1 {
        acc
    } else {
        acc.wrapping_neg()
    }
}

fn decompose_node(s: &str) -> (&str, i32, &str) {
    // Node decomposition / parsing rule (top-level split):
    // Scan with a single nesting counter incremented on '[' and '(' and decremented on ']' and ')'.
    // Select the first '[' encountered when counter == 0.
    // Left part: all chars before that '['.
    // Right part: all chars after the matching ']' that returns counter from 1 back to 0.
    let bytes = s.as_bytes();
    let mut depth: i32 = 0;

    let mut left_end: Option<usize> = None;
    let mut val_start: usize = 0;
    let mut val_end_bracket: usize = 0;

    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'[' || c == b'(' {
            if c == b'[' && depth == 0 && left_end.is_none() {
                left_end = Some(i);
                val_start = i + 1;
                // Now find the matching ']' that returns depth from 1 back to 0.
                depth += 1; // account for this '['
                i += 1;
                while i < bytes.len() {
                    let c2 = bytes[i];
                    if c2 == b'[' || c2 == b'(' {
                        depth += 1;
                    } else if c2 == b']' || c2 == b')' {
                        depth -= 1;
                        if c2 == b']' && depth == 0 {
                            val_end_bracket = i;
                            break;
                        }
                    }
                    i += 1;
                }
                break;
            } else {
                depth += 1;
            }
        } else if c == b']' || c == b')' {
            depth -= 1;
        }
        i += 1;
    }

    let le = left_end.expect("well-formed input expected");
    let left = &s[..le];
    let value_text = &s[val_start..val_end_bracket];
    let value = atoi_c_like(value_text);
    let right = &s[val_end_bracket + 1..];
    (left, value, right)
}

fn combine(a: &str, b: &str) -> String {
    // Combine two non-empty node strings (caller ensures not "()" at this level).
    let (al, av, ar) = decompose_node(a);
    let (bl, bv, br) = decompose_node(b);

    let sum = av.wrapping_add(bv);

    let left_combined = if is_empty_subtree(al) || is_empty_subtree(bl) {
        String::from("()")
    } else {
        let inner = combine(strip_one_outer_parens(al), strip_one_outer_parens(bl));
        format!("({})", inner)
    };

    let right_combined = if is_empty_subtree(ar) || is_empty_subtree(br) {
        String::from("()")
    } else {
        let inner = combine(strip_one_outer_parens(ar), strip_one_outer_parens(br));
        format!("({})", inner)
    };

    // Output construction: (<combined_left>)[<sum>](<combined_right>)
    // Note: left_combined/right_combined already include their outer parentheses.
    format!("{}[{}]{}", left_combined, sum, right_combined)
}

fn main() {
    // Read exactly two whitespace-delimited strings; ignore any additional input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let s1 = match it.next() {
        Some(x) => x,
        None => return,
    };
    let s2 = match it.next() {
        Some(x) => x,
        None => return,
    };

    // Specification implies inputs are well-formed non-empty trees at the top level.
    // If either is "()", the structural intersection is empty; however, the spec's
    // combination rule is defined for node positions. We interpret top-level similarly.
    let out = if is_empty_subtree(s1) || is_empty_subtree(s2) {
        String::from("()")
    } else {
        combine(s1, s2)
    };

    print!("{out}\n");
}
