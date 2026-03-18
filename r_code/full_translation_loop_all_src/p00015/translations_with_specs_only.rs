use std::io::{self, Read};

fn is_all_digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b'0' <= b && b <= b'9')
}

fn trim_leading_zeros(s: &str) -> &str {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() { "0" } else { trimmed }
}

fn add_decimal_strings(a: &str, b: &str) -> String {
    // a and b are assumed to be digit-only, non-empty.
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    let mut i: isize = ab.len() as isize - 1;
    let mut j: isize = bb.len() as isize - 1;
    let mut carry: u8 = 0;

    let mut out: Vec<u8> = Vec::with_capacity(ab.len().max(bb.len()) + 1);

    while i >= 0 || j >= 0 || carry != 0 {
        let da = if i >= 0 { ab[i as usize] - b'0' } else { 0 };
        let db = if j >= 0 { bb[j as usize] - b'0' } else { 0 };
        let sum = da + db + carry;
        out.push(b'0' + (sum % 10));
        carry = sum / 10;

        i -= 1;
        j -= 1;
    }

    out.reverse();
    String::from_utf8(out).unwrap()
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => return, // Precondition violated: no valid dataset count; produce no output.
    };

    let mut outputs: Vec<String> = Vec::with_capacity(t);

    for _ in 0..t {
        let a_raw = match it.next() {
            Some(s) => s,
            None => break, // Not enough tokens; stop (narrowest behavior).
        };
        let b_raw = match it.next() {
            Some(s) => s,
            None => break,
        };

        // Preconditions: valid non-negative integer strings.
        // If violated, treat as overflow per spec's postcondition wording.
        if !is_all_digits(a_raw) || !is_all_digits(b_raw) {
            outputs.push("overflow".to_string());
            continue;
        }

        // Length invariant: must not exceed 80 characters.
        if a_raw.len() > 80 || b_raw.len() > 80 {
            outputs.push("overflow".to_string());
            continue;
        }

        // Normalize inputs to avoid leading zeros in output (unless result is zero).
        let a = trim_leading_zeros(a_raw);
        let b = trim_leading_zeros(b_raw);

        let sum = add_decimal_strings(a, b);

        // Defined limit: since only explicit limit given is 80 characters, treat any sum
        // exceeding 80 digits as overflow.
        if sum.len() > 80 {
            outputs.push("overflow".to_string());
            continue;
        }

        // Ensure no leading zeros unless the result is zero.
        let sum_norm = trim_leading_zeros(&sum).to_string();
        outputs.push(sum_norm);
    }

    print!("{}", outputs.join("\n"));
}
