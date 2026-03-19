use std::io::{self, Read};

fn print_seq(out: &mut String, a: &[i64]) {
    for (i, v) in a.iter().enumerate() {
        if i != 0 {
            out.push(' ');
        }
        out.push_str(&v.to_string());
    }
    out.push('\n');
}

// Returns true if a lexicographically next permutation exists and is applied in-place.
fn next_permutation(a: &mut [i64]) -> bool {
    if a.len() < 2 {
        return false;
    }

    // Find rightmost i such that a[i] < a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..a.len() - 1).rev() {
        if a[i] < a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false, // non-increasing: no next permutation
    };

    // Find rightmost j > i such that a[j] > a[i]
    let mut j = a.len() - 1;
    while !(a[j] > a[i]) {
        j -= 1;
    }
    a.swap(i, j);

    // Reverse suffix i+1..end to get smallest suffix
    a[i + 1..].reverse();
    true
}

// Returns true if a lexicographically previous permutation exists and is applied in-place.
fn prev_permutation(a: &mut [i64]) -> bool {
    if a.len() < 2 {
        return false;
    }

    // Find rightmost i such that a[i] > a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..a.len() - 1).rev() {
        if a[i] > a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false, // non-decreasing: no previous permutation
    };

    // Find rightmost j > i such that a[j] < a[i]
    let mut j = a.len() - 1;
    while !(a[j] < a[i]) {
        j -= 1;
    }
    a.swap(i, j);

    // Reverse suffix i+1..end to get largest suffix
    a[i + 1..].reverse();
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let mut out = String::new();

    if n == 1 {
        // Specification: for length 1, output exactly one line containing `1`.
        out.push_str("1\n");
        print!("{}", out);
        return;
    }

    // Preserve original sequence exactly as read for the middle line.
    let original = a.clone();

    // Previous permutation line (if exists)
    let mut prev = a.clone();
    if prev_permutation(&mut prev) {
        print_seq(&mut out, &prev);
    }

    // Original sequence line
    print_seq(&mut out, &original);

    // Next permutation line (if exists)
    let mut next = a.clone();
    if next_permutation(&mut next) {
        print_seq(&mut out, &next);
    }

    print!("{}", out);
}
