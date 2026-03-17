use std::io::{self, BufRead};

fn compare(a: &i32, b: &i32) -> std::cmp::Ordering {
    a.cmp(b)
}

fn upper_bound(data: &[i32], key: i32) -> usize {
    let mut l = 0;
    let mut r = data.len() as isize - 1;

    while l <= r as usize {
        let m = (l + r as usize) / 2;

        if data[m] > key {
            r = m as isize - 1;
        } else {
            l = m + 1;
        }
    }

    l
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut b: Vec<i32> = Vec::with_capacity(m);
    for _ in 0..m {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        b.push(value);
    }

    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        c.push(value);
    }

    // Sort the array `a` using Rust's sort, which is stable and uses a safe comparison
    a.sort_by(compare);

    let mut sum: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1] as i64;
    }

    for i in 0..m {
        let idx = upper_bound(&a, b[i]);
        if idx > 0 && sum[idx] >= c[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
