use std::io::{self, Read};

fn reverse_range(s: &mut [u8], start: usize, n: usize) {
    // Mirrors C: for (i=0; i<n/2; ++i) swap s[start+i] and s[start+n-1-i]
    for i in 0..(n / 2) {
        let a = start + i;
        let b = start + (n - 1 - i);
        s.swap(a, b);
    }
}

fn swap_segments(s: &mut [u8], split: usize) {
    // C swap(s1, s2) where s2 = s1 + split:
    // result = s2..end + s1..s2
    let len = s.len();
    let mut t = Vec::with_capacity(len);
    t.extend_from_slice(&s[split..]);
    t.extend_from_slice(&s[..split]);
    s.copy_from_slice(&t);
}

fn contain(a: &[Vec<u8>], s: &[u8]) -> bool {
    a.iter().any(|x| x.as_slice() == s)
}

fn main() {
    // Read all input, tokenize like scanf("%d") / scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..n {
        let s_tok = it.next().unwrap_or("");
        // C uses char s[80]; scanf("%s", s); so token length is assumed to fit.
        // We preserve behavior for typical inputs; if longer, C would overflow (UB).
        let mut s = s_tok.as_bytes().to_vec();
        let t = s.clone();
        let len = s.len();

        let mut a: Vec<Vec<u8>> = Vec::new(); // stores unique strings encountered

        for i in 1..len {
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, 0, i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, i, len - i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, 0, i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            s = t.clone();

            swap_segments(&mut s, i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, 0, len - i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, len - i, i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            reverse_range(&mut s, 0, len - i);
            if !contain(&a, &s) {
                a.push(s.clone());
            }
            s = t.clone();
        }

        out.push_str(&format!("{}\n", a.len()));
    }

    print!("{out}");
}
