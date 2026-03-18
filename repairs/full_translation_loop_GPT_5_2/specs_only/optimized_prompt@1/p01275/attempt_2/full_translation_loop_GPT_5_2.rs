use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(tok) => tok.parse::<usize>().unwrap_or(0),
            None => break,
        };
        if n == 0 {
            break;
        }

        let s = match it.next() {
            Some(v) => v.as_bytes(),
            None => break,
        };
        let t = match it.next() {
            Some(v) => v.as_bytes(),
            None => break,
        };

        // Undefined behavior if lengths mismatch; assume valid per specification.
        // Operation model (narrowest consistent with sample):
        // One "rotation" chooses a position i and rotates the digit at i by +1 or -1 (mod 10).
        // Goal: minimum number of such single-digit rotations to transform s into t.
        // This reduces to sum over digits of min((t-s) mod 10, (s-t) mod 10).
        let mut ans: i32 = 0;
        for i in 0..n {
            let a = (s[i] - b'0') as i32;
            let b = (t[i] - b'0') as i32;
            let up = (b - a + 10) % 10;
            let down = (a - b + 10) % 10;
            ans += up.min(down);
        }

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{}", out);
}
