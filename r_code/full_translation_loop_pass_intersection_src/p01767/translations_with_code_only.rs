use std::io::{self, Read};

fn upper_bound(data: &[i32], key: i32) -> usize {
    // Mirrors the C implementation:
    // l = 0, r = n-1; while (l <= r) { m=(l+r)/2; if data[m] > key r=m-1 else l=m+1 } return l;
    let n = data.len();
    if n == 0 {
        return 0;
    }
    let mut l: i32 = 0;
    let mut r: i32 = n as i32 - 1;

    while l <= r {
        // In C, (l + r) / 2 uses signed int arithmetic.
        // Here l,r are non-negative and within bounds, so no overflow concerns.
        let m = (l + r) / 2;
        let mv = data[m as usize];
        if mv > key {
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
        None => return, // No input: C would have undefined behavior; we just exit.
    };

    let mut a: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    let mut b: Vec<i32> = Vec::with_capacity(m);
    for _ in 0..m {
        let v: i32 = it.next().unwrap().parse().unwrap();
        b.push(v);
    }

    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        c.push(v);
    }

    // C uses qsort with comparator `*(int*)a - *(int*)b`, which can overflow (UB).
    // We use a total order sort; for typical intended inputs this matches.
    a.sort();

    // sum[0]=0; sum[i]=sum[i-1]+a[i-1] for i=1..=n
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
