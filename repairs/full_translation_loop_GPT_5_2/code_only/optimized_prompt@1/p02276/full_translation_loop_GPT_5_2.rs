use std::io::{self, Read};

const N: usize = 1_000_000;

fn partition(a: &mut [i32], p: i32, r: i32) -> i32 {
    // Mirrors the C logic exactly (1-based indices used by caller).
    let x = a[r as usize];
    let mut i = p - 1;
    for j in p..r {
        if a[j as usize] <= x {
            i += 1;
            a.swap(i as usize, j as usize);
        }
    }
    a.swap((i + 1) as usize, r as usize);
    i + 1
}

fn main() {
    // Read all input, parse as i32 like C's "%d".
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };

    // Global C array is zero-initialized; emulate with a fixed-size Vec.
    // We keep indices 0..=N-1; the C code uses 1..=n.
    let mut a = vec![0i32; N];

    for idx in 1..=n {
        if let Some(s) = it.next() {
            a[idx as usize] = s.parse().unwrap();
        } else {
            // Missing input: C would leave A[idx] uninitialized (UB).
            // We keep the zero-initialized value, which is a reasonable fallback.
            break;
        }
    }

    let x = partition(&mut a, 1, n);

    let mut out = String::new();
    for i in 1..=n {
        if i == x {
            out.push('[');
            out.push_str(&a[i as usize].to_string());
            out.push(']');
        } else {
            out.push_str(&a[i as usize].to_string());
        }
        if i < n {
            out.push(' ');
        }
    }
    out.push('\n');
    print!("{out}");
}
