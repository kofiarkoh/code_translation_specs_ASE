use std::io::{self, BufRead};
use std::vec::Vec;

const SENTINEL: i64 = 1_000_000_000_000_000_000;

fn merge(A: &mut Vec<i64>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;

    let mut L = vec![0; n1 + 1];
    let mut R = vec![0; n2 + 1];

    for i in 0..n1 {
        L[i] = A[left + i];
    }
    for j in 0..n2 {
        R[j] = A[mid + j];
    }

    L[n1] = SENTINEL;
    R[n2] = SENTINEL;

    let mut i = 0;
    let mut j = 0;

    for k in left..right {
        if L[i] <= R[j] {
            A[k] = L[i];
            i += 1;
        } else {
            A[k] = R[j];
            j += 1;
        }
    }
}

fn merge_sort(A: &mut Vec<i64>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(A, left, mid);
        merge_sort(A, mid, right);
        merge(A, left, mid, right);
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let k: usize = nums.next().unwrap().parse().unwrap();

    let mut A = vec![0; n];
    let mut c = vec![0; n + 1];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        A[i] = line.trim().parse().unwrap();
        c[A[i]] += 1;
    }

    merge_sort(&mut c, 1, n + 1);

    let mut ans = 0;
    let mut t = 0;

    for i in 1..=n - k {
        if c[i] > 0 {
            ans += c[i];
            t += 1;
        }
    }

    println!("{}", ans);
}
