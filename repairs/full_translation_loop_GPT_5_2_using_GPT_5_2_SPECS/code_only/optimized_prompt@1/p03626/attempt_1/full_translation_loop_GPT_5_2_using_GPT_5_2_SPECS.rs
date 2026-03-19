use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace (matches scanf("%d") and scanf("%s%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let s1 = it.next().unwrap_or("").as_bytes().to_vec();
    let s2 = it.next().unwrap_or("").as_bytes().to_vec();

    // Emulate the C code's indexing behavior as closely as possible.
    // Note: The original C code has potential out-of-bounds access at S1[i+1] when i == N-1.
    // Here we avoid panics by treating out-of-range as 0, which is a best-effort approximation
    // without invoking undefined behavior in Rust.
    let get = |s: &Vec<u8>, idx: usize| -> u8 { *s.get(idx).unwrap_or(&0u8) };

    let mut ans: i64 = 1;
    let mut t: i32;

    if get(&s1, 0) == get(&s2, 0) {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    let m: i64 = 1_000_000_007;

    for i in 1..n {
        if get(&s1, i) == get(&s2, i) {
            if t == 1 {
                ans = (ans * 2) % m;
            }
            t = 1;
        }
        if get(&s1, i) == get(&s1, i + 1) {
            if t == 1 {
                ans = (ans * 2) % m;
            } else {
                ans = (ans * 3) % m;
            }
            t = 0;
        }
    }

    println!("{}", ans);
}
