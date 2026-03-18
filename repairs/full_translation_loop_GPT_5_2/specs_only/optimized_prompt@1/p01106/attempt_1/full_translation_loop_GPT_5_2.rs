use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: i64 = match n_opt.unwrap().parse() {
            Ok(v) => v,
            Err(_) => break, // Narrow interpretation: stop on malformed input.
        };

        if n <= 0 {
            break; // Termination condition: non-positive n produces no output for that set.
        }

        let i_tok = match it.next() {
            Some(v) => v,
            None => break,
        };
        let j_tok = match it.next() {
            Some(v) => v,
            None => break,
        };

        // "long long" in C is typically 64-bit signed; use i64.
        let i: i64 = match i_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let j: i64 = match j_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Specification does not define the exact mapping from (i, j) bits to 'L'/'R'.
        // Narrowest reasonable interpretation consistent with "based on the binary representation
        // of two integers" and producing exactly n characters:
        // For k = 0..n-1, compare the k-th least significant bits of i and j:
        // - output 'L' if they are equal
        // - output 'R' if they differ
        //
        // This uses only bit-level information from both integers and always yields 'L'/'R'.
        // Use u64 views to avoid sign-extension surprises when shifting negative i64.
        let iu = i as u64;
        let ju = j as u64;

        let mut s = String::with_capacity(n as usize);
        for k in 0..(n as u32) {
            let bi = (iu >> k) & 1;
            let bj = (ju >> k) & 1;
            s.push(if bi == bj { 'L' } else { 'R' });
        }

        out.push_str(&s);
        out.push('\n');
    }

    print!("{out}");
}
