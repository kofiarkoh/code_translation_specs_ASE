use std::io::{self, Read};

fn main() {
    // Read entire stdin and parse as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let q: usize = it.next().unwrap().parse().unwrap();

    // Temporary buffer proportional to maximum sequence size (<= 1000).
    let mut tmp: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..q {
        let b: usize = it.next().unwrap().parse().unwrap();
        let m: usize = it.next().unwrap().parse().unwrap();
        let e: usize = it.next().unwrap().parse().unwrap();

        // Preconditions are assumed; if violated, behavior is undefined per spec.
        // Rotate subrange [b, e) left by (m - b), i.e., make original index m first.
        tmp.clear();
        tmp.extend_from_slice(&a[m..e]);
        tmp.extend_from_slice(&a[b..m]);

        // Write back in-place.
        a[b..e].copy_from_slice(&tmp[..(e - b)]);
    }

    // Output: n integers separated by spaces, ending with newline.
    let mut out = String::new();
    for i in 0..n {
        if i > 0 {
            out.push(' ');
        }
        out.push_str(&a[i].to_string());
    }
    out.push('\n');
    print!("{out}");
}
