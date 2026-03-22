use std::io::{self, Read};

fn is_over_80_digits(s: &str) -> bool {
    s.as_bytes().len() > 80
}

fn add_decimal_strings(a: &str, b: &str) -> String {
    // Assumes (per spec) that all characters are ASCII digits '0'..'9'.
    // If not, numeric meaning is unspecified; we still perform a digit-based computation.
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    let mut i: isize = ab.len() as isize - 1;
    let mut j: isize = bb.len() as isize - 1;
    let mut carry: u8 = 0;

    // Result length is at most max(len(a), len(b)) + 1, and inputs are <= 80 here.
    let mut out_rev: Vec<u8> = Vec::with_capacity(ab.len().max(bb.len()) + 1);

    while i >= 0 || j >= 0 || carry != 0 {
        let da: u8 = if i >= 0 {
            ab[i as usize].wrapping_sub(b'0')
        } else {
            0
        };
        let db: u8 = if j >= 0 {
            bb[j as usize].wrapping_sub(b'0')
        } else {
            0
        };

        let sum: u16 = da as u16 + db as u16 + carry as u16;
        let digit: u8 = (sum % 10) as u8;
        carry = (sum / 10) as u8;

        out_rev.push(digit + b'0');

        i -= 1;
        j -= 1;
    }

    out_rev.reverse();

    // Remove leading zeros, but keep a single '0' if the number is zero.
    let mut k = 0usize;
    while k + 1 < out_rev.len() && out_rev[k] == b'0' {
        k += 1;
    }
    String::from_utf8(out_rev[k..].to_vec()).unwrap_or_else(|_| "0".to_string())
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap_or(0),
        None => return,
    };

    let mut outputs: Vec<String> = Vec::with_capacity(t);

    for _case_idx in 0..t {
        // Spec allows not-well-defined behavior on missing tokens; we choose a narrow,
        // deterministic behavior: treat missing tokens as empty strings.
        let a = it.next().unwrap_or("");
        let b = it.next().unwrap_or("");

        if is_over_80_digits(a) || is_over_80_digits(b) {
            outputs.push("overflow".to_string());
            continue;
        }

        let sum = add_decimal_strings(a, b);

        if sum.as_bytes().len() > 80 {
            outputs.push("overflow".to_string());
        } else {
            outputs.push(sum);
        }
    }

    print!("{}", outputs.join("\n"));
    if !outputs.is_empty() {
        print!("\n");
    }
}
