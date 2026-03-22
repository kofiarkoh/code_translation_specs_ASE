use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-delimited tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say this is present and parseable, and non-negative.
    let t: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap_or(0),
        None => return,
    };

    let mut out = String::new();

    for _case_idx in 0..t {
        // Specification: if fewer than two tokens are available, behavior is not inferable.
        // We choose the narrowest reasonable, safe behavior: stop processing further cases.
        let a = match it.next() {
            Some(s) => s,
            None => break,
        };
        let b = match it.next() {
            Some(s) => s,
            None => break,
        };

        // Overflow rule: if either token length exceeds 80, print overflow indication.
        if a.len() > 80 || b.len() > 80 {
            out.push_str("overflow\n");
            continue;
        }

        // Add two non-negative decimal integers given as digit strings.
        // Preconditions guarantee digits only; we do not add extra validation.
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        let mut i = a_bytes.len();
        let mut j = b_bytes.len();
        let mut carry: u8 = 0;

        // Build result in reverse (least significant digit first).
        let mut rev_digits: Vec<u8> = Vec::with_capacity(81);

        while i > 0 || j > 0 || carry != 0 {
            let da = if i > 0 {
                i -= 1;
                a_bytes[i] - b'0'
            } else {
                0
            };
            let db = if j > 0 {
                j -= 1;
                b_bytes[j] - b'0'
            } else {
                0
            };

            let sum = da as u16 + db as u16 + carry as u16;
            rev_digits.push((sum % 10) as u8);
            carry = (sum / 10) as u8;

            // If the exact sum requires more than 80 digits, overflow.
            if rev_digits.len() > 80 {
                break;
            }
        }

        if rev_digits.len() > 80 {
            out.push_str("overflow\n");
            continue;
        }

        // Convert to normal order and strip leading zeros (except print "0" for zero).
        // Note: For correct addition, leading zeros can only occur if result is zero.
        let mut k = rev_digits.len();
        while k > 1 && rev_digits[k - 1] == 0 {
            k -= 1;
        }

        for idx in (0..k).rev() {
            out.push((b'0' + rev_digits[idx]) as char);
        }
        out.push('\n');
    }

    print!("{out}");
}
