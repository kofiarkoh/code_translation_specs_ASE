use std::io::{self, Read};

fn upper_bound(data: &[i32], key: i32) -> usize {
    // Mirrors the C implementation:
    // l=0, r=n-1; while (l<=r) { m=(l+r)/2; if data[m]>key r=m-1 else l=m+1 } return l;
    // Use i64 indices to avoid underflow when r becomes -1.
    let mut l: i64 = 0;
    let mut r: i64 = data.len() as i64 - 1;

    while l <= r {
        let m = (l + r) / 2;
        if data[m as usize] > key {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    l as usize
}

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    let mut b: Vec<i32> = Vec::with_capacity(m);
    for _ in 0..m {
        b.push(it.next().unwrap().parse::<i32>().unwrap());
    }

    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        c.push(it.next().unwrap().parse::<i64>().unwrap());
    }

    // C uses qsort with comparator `*(int*)a - *(int*)b`, which can overflow (UB in C).
    // We choose a normal numeric sort; this matches intended ascending order.
    a.sort_unstable();

    // sum[0]=0; for i=1..=n sum[i]=sum[i-1]+a[i-1]
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
