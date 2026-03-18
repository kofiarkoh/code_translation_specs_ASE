use std::io::{self, BufRead};

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
    
    // Read n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for n");
    let mut a: Vec<i32> = Vec::with_capacity(n);
    
    // Read array a
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for array a");
        a.push(value);
    }

    // Read m
    let m: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for m");
    let mut b: Vec<i32> = Vec::with_capacity(m);
    let mut c: Vec<i64> = Vec::with_capacity(m);
    
    // Read array b
    for _ in 0..m {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for array b");
        b.push(value);
    }

    // Read array c
    for _ in 0..m {
        let value: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for array c");
        c.push(value);
    }

    // Sort array a
    a.sort_unstable();

    // Compute cumulative sums
    let mut sum: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1] as i64;
    }

    // Output results
    for i in 0..m {
        let index = upper_bound(&a, b[i]);
        if index <= n && sum[index] >= c[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
