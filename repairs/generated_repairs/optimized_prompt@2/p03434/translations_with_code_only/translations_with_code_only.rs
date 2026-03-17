use std::io::{self, BufRead};

const SENTINEL: i64 = 1_000_000_000_000_000_000;

fn merge(a: &mut Vec<i64>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;

    let mut l = vec![0; n1 + 1];
    let mut r = vec![0; n2 + 1];

    for i in 0..n1 {
        l[i] = a[left + i];
    }
    for j in 0..n2 {
        r[j] = a[mid + j];
    }

    l[n1] = SENTINEL;
    r[n2] = SENTINEL;

    let mut i = 0;
    let mut j = 0;

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

fn merge_sort(a: &mut Vec<i64>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(a, left, mid);
        merge_sort(a, mid, right);
        merge(a, left, mid, right);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap_or(0);
    let mut a: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(value) = line.parse::<i64>() {
                a.push(value);
            }
        }
    }

    merge_sort(&mut a, 0, n);

    let (mut a_sum, mut b_sum) = (0, 0);

    if n % 2 == 0 {
        for i in (0..n).rev().step_by(2) {
            a_sum += a[i];
            b_sum += a[i - 1];
        }
        println!("{}", a_sum - b_sum);
    } else {
        for i in (1..n).rev().step_by(2) {
            a_sum += a[i];
            b_sum += a[i - 1];
        }
        a_sum += a[0];
        println!("{}", a_sum - b_sum);
    }
}
