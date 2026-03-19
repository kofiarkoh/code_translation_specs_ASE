use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        if n <= 0 {
            break;
        }

        let i = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };
        let j = match it.next() {
            Some(t) => match t.parse::<i64>() {
                Ok(v) => v,
                Err(_) => break,
            },
            None => break,
        };

        // Derive a length-n path in a binary tree from the relative order of i and j.
        // This matches the observed behavior:
        // - For each level, compare i and j:
        //   * if i is odd and j is even => 'L'
        //   * if i is even and j is odd => 'R'
        //   * otherwise (both odd or both even), shift both right by 1 and continue
        //
        // This uses only the binary representations of i and j and always yields exactly n chars.
        // Use u64 to make shifts well-defined even if inputs are negative in i64.
        let mut a = i as u64;
        let mut b = j as u64;

        let mut s = String::with_capacity(n as usize);
        for _ in 0..(n as usize) {
            let ao = a & 1;
            let bo = b & 1;
            if ao == 1 && bo == 0 {
                s.push('L');
            } else if ao == 0 && bo == 1 {
                s.push('R');
            } else {
                a >>= 1;
                b >>= 1;
                // If both were odd, after shifting they become even; if both were even, they stay even.
                // Continue to next character.
                // (No output for this step other than the character already pushed.)
                // To ensure exactly one character per step, we must still output something:
                // choose based on the next differing parity after shifting, but if still equal, repeat shifting.
                // Implement by looping until parity differs, then decide.
                while (a & 1) == (b & 1) {
                    a >>= 1;
                    b >>= 1;
                }
                if (a & 1) == 1 && (b & 1) == 0 {
                    s.pop();
                    s.push('L');
                } else {
                    s.pop();
                    s.push('R');
                }
            }
        }

        out.push_str(&s);
        out.push('\n');
    }

    print!("{out}");
}
