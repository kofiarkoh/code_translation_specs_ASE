use std::io::{self, Read};

const SENTINEL: i64 = 1_000_000_000_000_000_000;

fn merge(a: &mut [i64], left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;

    // Allocate temporary arrays with sentinel at the end (as in the C code).
    let mut l = Vec::with_capacity(n1 + 1);
    let mut r = Vec::with_capacity(n2 + 1);

    l.extend_from_slice(&a[left..mid]);
    r.extend_from_slice(&a[mid..right]);

    l.push(SENTINEL);
    r.push(SENTINEL);

    let mut i = 0usize;
    let mut j = 0usize;

    for k in left..right {
        if l[i] <= r[j] {
            a[k] = l[i];
            i += 1;
        } else {
            a[k] = r[j];
            j += 1;
        }
    }
}

fn merge_sort(a: &mut [i64], left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(a, left, mid);
        merge_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}

fn main() {
    // Read all input (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    merge_sort(&mut a, 0, n);

    // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
    let mut sum_a: i64 = 0;
    let mut sum_b: i64 = 0;

    if n % 2 == 0 {
        let mut i = n - 1;
        loop {
            sum_a = sum_a.wrapping_add(a[i]);
            sum_b = sum_b.wrapping_add(a[i - 1]);
            if i < 2 {
                break;
            }
            i -= 2;
        }
    } else {
        let mut i = n - 1;
        while i >= 1 {
            sum_a = sum_a.wrapping_add(a[i]);
            sum_b = sum_b.wrapping_add(a[i - 1]);
            if i < 2 {
                break;
            }
            i -= 2;
        }
        sum_a = sum_a.wrapping_add(a[0]);
    }

    let ans = sum_a.wrapping_sub(sum_b);
    print!("{}\n", ans);
}
