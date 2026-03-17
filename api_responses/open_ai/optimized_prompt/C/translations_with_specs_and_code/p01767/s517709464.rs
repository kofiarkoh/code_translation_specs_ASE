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
    
    // Read size of the first array
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut a: Vec<i32> = Vec::with_capacity(n);
    
    // Read the first array
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Read size of the second array
    let m: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut b: Vec<i32> = Vec::with_capacity(m);
    
    // Read the second array
    for _ in 0..m {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        b.push(value);
    }

    // Read the long long integer array
    let mut c: Vec<i64> = Vec::with_capacity(m);
    for _ in 0..m {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        c.push(value);
    }

    // Sort the first array
    a.sort_by(compare);

    // Compute cumulative sums
    let mut sum: Vec<i64> = vec![0; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i - 1] as i64;
    }

    // Generate output
    for i in 0..m {
        let index = upper_bound(&a, b[i]);
        if sum[index] >= c[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
