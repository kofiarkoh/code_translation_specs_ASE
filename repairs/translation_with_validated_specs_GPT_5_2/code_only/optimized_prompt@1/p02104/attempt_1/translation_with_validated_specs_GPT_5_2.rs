use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    // Emulate the C in(): assumes next chars are digits; stops when c < '0'
    fn in_num(bytes: &[u8], idx: &mut usize) -> i32 {
        let mut n: i32 = 0;
        // In the original C, it immediately reads a char and starts parsing digits.
        // We mimic that behavior; if input is malformed, behavior is unspecified anyway.
        while *idx < bytes.len() && bytes[*idx] < b'0' {
            // The C code doesn't skip non-digits; it would treat them as terminators
            // only after reading at least one digit. However AOJ input is well-formed.
            // We skip leading whitespace to be robust without changing intended behavior.
            *idx += 1;
        }
        while *idx < bytes.len() {
            let c = bytes[*idx];
            if c < b'0' {
                break;
            }
            n = (n << 3) + (n << 1) + ((c & 0x0f) as i32);
            *idx += 1;
        }
        n
    }

    let n = in_num(bytes, &mut idx) as usize;

    // 1-based arrays sized n+1
    let mut p = vec![0i32; n + 1];
    let mut hi = vec![0usize; n + 1];
    let mut ans = vec![0i32; n + 1];

    // Read p[i] and count occurrences in hi[a]
    for i in 1..=n {
        let a = in_num(bytes, &mut idx) as usize;
        p[i] = a as i32;
        if a <= n {
            hi[a] += 1;
        } else {
            // C would index out of bounds (UB) if a > n; input is assumed valid.
            // Keep behavior defined in Rust by ignoring; but AOJ constraints should prevent this.
        }
    }

    // Allocate q[i] with length hi[i]
    let mut q: Vec<Vec<i32>> = vec![Vec::new(); n + 1];
    for i in 1..=n {
        q[i] = Vec::with_capacity(hi[i]);
    }

    // memset(hi, 0, (n+1)<<2);
    for v in hi.iter_mut() {
        *v = 0;
    }

    // Fill q[a] with indices i, using hi[a] as current length counter
    for i in 1..=n {
        let a = p[i] as usize;
        let pos = hi[a];
        hi[a] = pos + 1;
        // Equivalent to q[a][pos] = i in C after malloc; we push in order.
        q[a].push(i as i32);
    }

    // Stack sp
    let mut sp: Vec<i32> = Vec::with_capacity(n);
    let mut top: usize = 0;

    // Two passes
    for _j in 0..2 {
        for i in 1..=n {
            // while (hi[i]) sp[top++] = q[i][--hi[i]];
            while hi[i] != 0 {
                hi[i] -= 1;
                let v = q[i][hi[i]];
                if top == sp.len() {
                    sp.push(v);
                } else {
                    sp[top] = v;
                }
                top += 1;
            }
            // if (!ans[i] && top) ans[i] = sp[--top];
            if ans[i] == 0 && top != 0 {
                top -= 1;
                ans[i] = sp[top];
            }
        }
    }

    // Output
    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
