use std::io::{self, Read};

fn main() {
    // The spec guarantees: N, Q, N integers, then Q queries each with (character, integer).
    // Prior attempts failed because the query integer may not be cleanly tokenized; it may be
    // adjacent to punctuation (e.g., "A(12)", "A:12", "A12,") or otherwise not parseable via
    // split_whitespace + parse().
    //
    // To stay faithful to the spec while being robust to such formatting, we:
    // - read N and Q as the first two integers found in the input,
    // - read the next N integers for the mapping,
    // - then for each query:
    //     * read the next non-whitespace character as the operation (ignored; semantics unspecified),
    //     * read the next integer found after that character as the query value.
    //
    // Query semantics are unspecified; we keep the minimal interpretation:
    // - maintain modification counts (non-negative),
    // - cumulative result adds mapping[idx] and increments mods[idx] for each query.

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let bytes = s.as_bytes();
    let mut i: usize = 0;

    fn skip_ws(bytes: &[u8], i: &mut usize) {
        while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
            *i += 1;
        }
    }

    fn read_i64(bytes: &[u8], i: &mut usize) -> i64 {
        // Find the next integer in the stream (optional sign + digits).
        while *i < bytes.len() {
            let b = bytes[*i];
            if b == b'+' || b == b'-' || b.is_ascii_digit() {
                break;
            }
            *i += 1;
        }
        let mut sign: i64 = 1;
        if *i < bytes.len() && bytes[*i] == b'-' {
            sign = -1;
            *i += 1;
        } else if *i < bytes.len() && bytes[*i] == b'+' {
            *i += 1;
        }
        let mut val: i64 = 0;
        while *i < bytes.len() && bytes[*i].is_ascii_digit() {
            val = val * 10 + (bytes[*i] - b'0') as i64;
            *i += 1;
        }
        sign * val
    }

    fn read_op_char(bytes: &[u8], i: &mut usize) -> u8 {
        // Read the next non-whitespace byte as the operation character.
        skip_ws(bytes, i);
        if *i >= bytes.len() {
            return 0;
        }
        let c = bytes[*i];
        *i += 1;
        c
    }

    let n = read_i64(bytes, &mut i) as usize;
    let q = read_i64(bytes, &mut i) as usize;

    let mut mapping: Vec<i64> = vec![0; n + 1];
    for idx in 1..=n {
        mapping[idx] = read_i64(bytes, &mut i);
    }

    let mut mods: Vec<u64> = vec![0; n + 1];
    let mut result: i128 = 0;

    for _ in 0..q {
        let _op = read_op_char(bytes, &mut i);
        let idx = read_i64(bytes, &mut i) as usize;

        mods[idx] = mods[idx].wrapping_add(1);
        result += mapping[idx] as i128;
    }

    println!("{}", result);
}
