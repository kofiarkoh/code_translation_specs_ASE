use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s_str = it.next().unwrap();
    // Per spec, n matches the actual length; behavior is undefined otherwise.
    let s = s_str.as_bytes();

    let q: usize = it.next().unwrap().parse().unwrap();
    let mut ks: Vec<usize> = Vec::with_capacity(q);
    for _ in 0..q {
        let k: usize = it.next().unwrap().parse().unwrap();
        ks.push(k);
    }

    let mut out = String::new();

    for &k in &ks {
        // Matches C behavior under precondition 0 <= k <= n.
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
                _ => {}
            }
        }

        // Slide window ending at i (inclusive), window is [i-k+1, i]
        for i in k..n {
            let out_ch = s[i - k];
            if out_ch == b'D' {
                // Remove contributions of this 'D' to all 'M' currently in window.
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
