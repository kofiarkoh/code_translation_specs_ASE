use std::io::{self, Read};

fn partition(a: &mut [i32], p: usize, r: usize) -> usize {
    // Mirrors the C code's Lomuto partition:
    // pivot = a[r], i = p-1, for j in [p..r-1], swap when a[j] <= pivot,
    // then swap pivot into i+1 and return i+1.
    let x = a[r];
    let mut i: isize = p as isize - 1;

    for j in p..r {
        if a[j] <= x {
            i += 1;
            a.swap(i as usize, j);
        }
    }
    let pivot_idx = (i + 1) as usize;
    a.swap(pivot_idx, r);
    pivot_idx
}

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // C code uses 1-based indexing with a global array; emulate with a dummy at index 0.
    let mut a: Vec<i32> = vec![0; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let pivot_pos = partition(&mut a, 1, n); // 1-based index in this vector

    let mut out = String::new();
    for i in 1..=n {
        if i == pivot_pos {
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
