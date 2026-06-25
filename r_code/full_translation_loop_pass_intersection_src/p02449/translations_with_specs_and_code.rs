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

// Computes the lexicographically previous permutation in-place.
// Returns true if such permutation exists, false if already smallest.
fn prev_permutation(a: &mut [i32]) -> bool {
    if a.len() <= 1 {
        return false;
    }
    // Find rightmost i such that a[i] > a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..=a.len() - 2).rev() {
        if a[i] > a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false,
    };

    // Find rightmost j > i such that a[j] < a[i], swap
    for j in (i + 1..a.len()).rev() {
        if a[j] < a[i] {
            a.swap(i, j);
            break;
        }
    }

    // Reverse suffix a[i+1..]
    a[i + 1..].reverse();
    true
}

// Computes the lexicographically next permutation in-place.
// Returns true if such permutation exists, false if already largest.
fn next_permutation(a: &mut [i32]) -> bool {
    if a.len() <= 1 {
        return false;
    }
    // Find rightmost i such that a[i] < a[i+1]
    let mut i_opt: Option<usize> = None;
    for i in (0..=a.len() - 2).rev() {
        if a[i] < a[i + 1] {
            i_opt = Some(i);
            break;
        }
    }
    let i = match i_opt {
        Some(i) => i,
        None => return false,
    };

    // Find rightmost j > i such that a[j] > a[i], swap
    for j in (i + 1..a.len()).rev() {
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
    // No explicit error handling per spec; assume valid input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    if n == 1 {
        // Per spec: exactly one line containing "1"
        print!("1\n");
        return;
    }

    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut prev_a = a.clone();
    let mut next_a = a.clone();

    let mut out = String::new();

    if prev_permutation(&mut prev_a) {
        print_seq(&mut out, &prev_a);
    }

    print_seq(&mut out, &a);

    if next_permutation(&mut next_a) {
        print_seq(&mut out, &next_a);
    }

    print!("{out}");
}
