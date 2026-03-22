use std::io::{self, Read};

fn upper_bound(data: &[i32], key: i32) -> usize {
    // Returns the first index i such that data[i] > key (i.e., count of elements <= key).
    let mut l: usize = 0;
    let mut r: usize = data.len();
    while l < r {
        let m = l + (r - l) / 2;
        if data[m] > key {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

fn main() {
    // Read entire stdin; behavior is unspecified if input is malformed/insufficient (per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let m: usize = it.next().unwrap().parse().unwrap();
    let mut b: Vec<i32> = Vec::with_capacity(m);
    for _ in 0..m {
        b.push(it.next().unwrap().parse().unwrap());
    }

    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        c.push(it.next().unwrap().parse().unwrap());
    }

    // Spec requires ordering without overflow in comparisons.
    // C code used subtraction in comparator (can overflow); Rust uses a safe total order.
    a.sort_unstable();

    // Prefix sums: sum[i] = sum of first i elements (i in 0..=n)
    let mut sum: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1] as i64;
    }

    let mut out = String::new();
    for i in 0..m {
        let idx = upper_bound(&a, b[i]);
        if sum[idx] >= c[i] {
            out.push_str("Yes\n");
        } else {
            out.push_str("No\n");
        }
    }

    print!("{out}");
}
