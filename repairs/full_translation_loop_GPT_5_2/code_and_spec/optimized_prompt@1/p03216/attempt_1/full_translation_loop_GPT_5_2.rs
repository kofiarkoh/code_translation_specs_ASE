use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace (matches scanf behavior for these inputs).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s_str = it.next().unwrap();
    let s = s_str.as_bytes(); // ASCII 'D','M','C' per spec
    let q: usize = it.next().unwrap().parse().unwrap();

    let mut ks: Vec<usize> = Vec::with_capacity(q);
    for _ in 0..q {
        ks.push(it.next().unwrap().parse().unwrap());
    }

    let mut out = String::new();

    for &k in &ks {
        let mut ans: i64 = 0;
        let mut d: i64 = 0;
        let mut m: i64 = 0;
        let mut dm: i64 = 0;

        // Initialize first window [0, k)
        for i in 0..k {
            match s[i] {
                b'D' => d += 1,
                b'M' => {
                    m += 1;
                    dm += d;
                }
                b'C' => ans += dm,
                _ => {} // undefined behavior per spec for invalid chars; ignore here
            }
        }

        // Slide window; note that ans counts contributions from all positions i>=0,
        // while d/m/dm are maintained for the last k characters.
        for i in k..n {
            let out_ch = s[i - k];
            if out_ch == b'D' {
                dm -= m;
                d -= 1;
            }
            if out_ch == b'M' {
                m -= 1;
            }

            match s[i] {
                b'D' => d += 1,
                b'M' => {
                    m += 1;
                    dm += d;
                }
                b'C' => ans += dm,
                _ => {}
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
