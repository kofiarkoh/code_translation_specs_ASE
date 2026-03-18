use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // If no input, do nothing (scanf would fail; behavior unspecified).
    };

    // C uses VLA `int s[n]`; in Rust use Vec with length n (if n <= 0, it's an empty vec).
    let n_usize = if n > 0 { n as usize } else { 0usize };
    let mut s: Vec<i32> = vec![0; n_usize];

    let mut k: i32 = 0;
    let mut minv: i32 = 101;

    for i in 0..n_usize {
        let val: i32 = it.next().unwrap().parse().unwrap();
        s[i] = val;

        // C signed overflow is UB; use wrapping to emulate typical two's complement behavior.
        k = k.wrapping_add(s[i]);

        if s[i] % 10 == 0 {
            s[i] = 101;
        }
        if minv > s[i] {
            minv = s[i];
        }
    }

    let mut ans: i32 = k;
    if k % 10 == 0 {
        if minv == 101 {
            ans = 0;
        } else {
            ans = k.wrapping_sub(minv);
        }
    }

    print!("{}\n", ans);
}
