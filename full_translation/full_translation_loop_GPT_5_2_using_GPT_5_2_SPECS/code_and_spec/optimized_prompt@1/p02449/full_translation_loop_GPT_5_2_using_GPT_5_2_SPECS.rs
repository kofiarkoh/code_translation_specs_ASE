use std::io::{self, Read};

fn print_seq(out: &mut String, a: &[i32]) {
    if a.is_empty() {
        return;
    }
    out.push_str(&a[0].to_string());
    for v in &a[1..] {
        out.push(' ');
        out.push_str(&v.to_string());
    }
    out.push('\n');
}

// Returns true and mutates `a` to the lexicographically previous permutation if it exists.
// Otherwise returns false and leaves `a` in an unspecified state (matches typical behavior;
// in our usage we only print when true).
fn prev_permutation(a: &mut [i32]) -> bool {
    let n = a.len();
    if n <= 1 {
        return false;
    }

    // Find rightmost i such that a[i] > a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..=n - 2).rev() {
        if a[i] > a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false, // non-decreasing => no previous permutation
    };

    // Find rightmost j > i such that a[j] < a[i], then swap
    for j in (i + 1..n).rev() {
        if a[j] < a[i] {
            a.swap(i, j);
            break;
        }
    }

    // Reverse suffix a[i+1..]
    a[i + 1..].reverse();
    true
}

// Returns true and mutates `a` to the lexicographically next permutation if it exists.
fn next_permutation(a: &mut [i32]) -> bool {
    let n = a.len();
    if n <= 1 {
        return false;
    }

    // Find rightmost i such that a[i] < a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..=n - 2).rev() {
        if a[i] < a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false, // non-increasing => no next permutation
    };

    // Find rightmost j > i such that a[j] > a[i], then swap
    for j in (i + 1..n).rev() {
        if a[j] > a[i] {
            a.swap(i, j);
            break;
        }
    }

    // Reverse suffix a[i+1..]
    a[i + 1..].reverse();
    true
}

fn main() {
    // No explicit error handling per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    if n == 1 {
        // Spec: for length 1, output exactly "1\n"
        print!("1\n");
        return;
    }

    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let mut out = String::new();

    // Previous permutation line (only if exists)
    let mut prev_a = a.clone();
    if prev_permutation(&mut prev_a) {
        print_seq(&mut out, &prev_a);
    }

    // Original sequence line (always for n > 1)
    print_seq(&mut out, &a);

    // Next permutation line (only if exists)
    let mut next_a = a.clone();
    if next_permutation(&mut next_a) {
        print_seq(&mut out, &next_a);
    }

    print!("{out}");
}
