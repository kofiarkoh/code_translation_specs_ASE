use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Place values for up to 3 digits, aligned as in the C code.
    // Index used: 3 - N + i, where i is 0-based position in the N-digit number.
    let d = [100i32, 10i32, 1i32];

    let mut num: i32 = 0;

    // Store first constraint pair to apply the "leading-zero rule" later.
    // In the C code, s[0], c[0] are read only when M>0.
    let mut first_s: i32 = 0;
    let mut first_c: i32 = 0;

    // digit[i] stores either -1 (unconstrained) or the contribution (digit * place value)
    // for position i.
    let mut digit = vec![-1i32; n];

    if m == 0 {
        // Per spec: output smallest non-negative integer with exactly N digits
        // (except N=1 yields 0).
        num = match n {
            1 => 0,
            2 => 10,
            3 => 100,
            _ => 0, // behavior outside {1,2,3} not inferable; keep benign default
        };
    } else {
        for i in 0..m {
            let s: i32 = it.next().unwrap().parse().unwrap();
            let c: i32 = it.next().unwrap().parse().unwrap();

            if i == 0 {
                first_s = s;
                first_c = c;
            }

            let pos = (s - 1) as usize;
            let place_idx = 3usize - n + pos;
            let place = d[place_idx];

            if digit[pos] == -1 {
                digit[pos] = c * place;
            } else if digit[pos] / place != c {
                num = -1; // contradiction
            }
        }

        if num != -1 {
            for i in 0..n {
                if digit[i] != -1 {
                    num += digit[i];
                } else if n > 1 && i == 0 {
                    // Leading digit unconstrained: default to 1 (i.e., add its place value).
                    // Note: the C code has a redundant `digit[i] != 0` check here; since
                    // digit[i] == -1 in this branch, it always holds. Spec says default to 1.
                    let place_idx = 3usize - n + i;
                    num += d[place_idx];
                } // other unconstrained digits default to 0 (add nothing)
            }
        }
    }

    // Leading-zero rule: applied only based on the first provided constraint pair.
    if n > 1 && first_s == 1 && first_c == 0 {
        // Mismatch note: This rule can override even a previously detected contradiction,
        // but both outcomes are "-1" anyway. We follow the spec/C behavior exactly.
        print!("-1");
    } else {
        print!("{}", num);
    }
}
