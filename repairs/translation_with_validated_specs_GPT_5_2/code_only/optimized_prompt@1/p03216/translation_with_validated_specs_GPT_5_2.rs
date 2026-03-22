use std::io::{self, Read};

fn main() {
    // Read entire stdin, tokenize by ASCII whitespace (matches scanf behavior for %d and %s).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i32>().unwrap() as usize,
        None => return,
    };

    let s_str = it.next().unwrap_or("");
    let s = s_str.as_bytes(); // treat as raw bytes; C code compares to 'D','M','C'

    let q: usize = it.next().unwrap().parse::<i32>().unwrap() as usize;

    let mut k: Vec<usize> = Vec::with_capacity(q);
    for _ in 0..q {
        let val = it.next().unwrap().parse::<i32>().unwrap() as usize;
        k.push(val);
    }

    let mut out = String::new();

    for &kj in &k {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        // The original C code assumes 0 <= kj <= n and s has at least n chars.
        // We preserve that assumption; if violated, behavior is unspecified in C anyway.
        for i in 0..kj {
            let ch = s[i];
            if ch == b'D' {
                d += 1;
            }
            if ch == b'M' {
                m += 1;
                dm += d;
            }
            if ch == b'C' {
                ans += dm;
            }
        }

        for i in kj..n {
            let out_ch = s[i - kj];
            if out_ch == b'D' {
                dm -= m;
                d -= 1;
            }
            if out_ch == b'M' {
                m -= 1;
            }

            let in_ch = s[i];
            if in_ch == b'D' {
                d += 1;
            }
            if in_ch == b'M' {
                m += 1;
                dm += d;
            }
            if in_ch == b'C' {
                ans += dm;
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
