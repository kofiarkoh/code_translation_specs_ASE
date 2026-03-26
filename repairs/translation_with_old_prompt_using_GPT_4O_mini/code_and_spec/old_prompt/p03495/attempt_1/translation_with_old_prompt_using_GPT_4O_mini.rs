use std::io::{self, BufRead};

const SENTINEL: i64 = 1_000_000_000_000_000_000;

fn merge(arr: &mut Vec<i64>, left: usize, mid: usize, right: usize) {
    let n1 = mid - left;
    let n2 = right - mid;

    let mut left_arr: Vec<i64> = arr[left..mid].to_vec();
    let mut right_arr: Vec<i64> = arr[mid..right].to_vec();

    left_arr.push(SENTINEL);
    right_arr.push(SENTINEL);

    let (mut i, mut j) = (0, 0);
    for k in left..right {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
    }
}

fn merge_sort(arr: &mut Vec<i64>, left: usize, right: usize) {
    if left + 1 < right {
        let mid = (left + right) / 2;
        merge_sort(arr, left, mid);
        merge_sort(arr, mid, right);
        merge(arr, left, mid, right);
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

    let mut a = vec![0; n];
    let mut c = vec![0; n + 1];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        a[i] = line.trim().parse().unwrap();
        c[a[i]] += 1;
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
