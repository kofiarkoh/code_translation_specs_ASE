use std::io::{self, Read};

const N: usize = 1_000_000;

fn partition(a: &mut [i32], p: usize, r: usize) -> usize {
    // Mirrors the C code's Lomuto partition on 1-based indices [p..=r].
    let x = a[r];
    let mut i: isize = p as isize - 1;

    for j in p..r {
        if a[j] <= x {
            i += 1;
            let iu = i as usize;
            a.swap(iu, j);
        }
    }
    let pivot_index = (i + 1) as usize;
    a.swap(pivot_index, r);
    pivot_index
}

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // The C program assumes 1 <= n <= N and uses 1-based indexing.
    // Allocate N+1 to allow indices 1..=n.
    let mut a = vec![0i32; N + 1];

    for i in 1..=n {
        if let Some(t) = it.next() {
            a[i] = t.parse().unwrap();
        } else {
            // Spec says inputs are valid; if missing, leave default 0.
            // (No explicit error behavior is specified.)
            break;
        }
    }

    let x = partition(&mut a, 1, n);

    let mut out = String::new();
    for i in 1..=n {
        if i == x {
            out.push('[');
            out.push_str(&a[i].to_string());
            out.push(']');
        } else {
            out.push_str(&a[i].to_string());
        }
        if i < n {
            out.push(' ');
        }
    }
    out.push('\n');

    print!("{out}");
}
