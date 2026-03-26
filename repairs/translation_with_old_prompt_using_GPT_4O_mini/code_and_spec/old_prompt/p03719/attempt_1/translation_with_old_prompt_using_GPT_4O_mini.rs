use std::io::{self, BufRead};

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

fn ngcd(n: usize, a: &[i32]) -> i32 {
    let mut d = a[0];
    for &value in &a[1..n] {
        if d == 1 {
            break;
        }
        d = gcd(value, d);
    }
    d
}

fn quicksort(a: &mut [char], first: usize, last: usize) {
    let pivot = a[(first + last) / 2];
    let (mut i, mut j) = (first, last);
    
    loop {
        while a[i] < pivot { i += 1; }
        while pivot < a[j] { j -= 1; }
        if i >= j { break; }
        a.swap(i, j);
        i += 1;
        j -= 1;
    }
    
    if first < i - 1 {
        quicksort(a, first, i - 1);
    }
    if j + 1 < last {
        quicksort(a, j + 1, last);
    }
}

fn is_prime(x: i32) -> i32 {
    if x <= 1 {
        return 0;
    }
    for i in 2..=((x as f64).sqrt() as i32) {
        if x % i == 0 {
            return 0;
        }
    }
    1
}

fn is_substr(s: &str, t: &str, i: usize) -> i32 {
    if i >= s.len() {
        return 0;
    }
    let s_slice = &s[i..];
    if s_slice.starts_with(t) {
        1
    } else {
        0
    }
}

fn factorize(ftr: &mut Vec<i32>, n: i32) -> usize {
    let mut n = n;
    let mut a = 2;
    let mut cnt = 0;

    while n >= a {
        if n % a == 0 {
            n /= a;
            ftr.push(a);
            cnt += 1;
        } else {
            a += 1;
        }
    }
    cnt
}

fn nextperm(p: &mut Vec<i32>, n: usize) -> i32 {
    let mut i = n - 1;
    p[0] = 0; // Sentinel
    while i > 0 && p[i] >= p[i + 1] {
        i -= 1;
    }
    if i == 0 {
        return 0; // Completed
    }
    let mut j = n;
    while p[i] >= p[j] {
        j -= 1;
    }
    p.swap(i, j);
    i += 1;
    j = n;
    while i < j {
        p.swap(i, j);
        i += 1;
        j -= 1;
    }
    1 // Not completed
}

fn comb(size1: usize, size2: usize, v: &mut Vec<Vec<i32>>) {
    for i in 0..size1 {
        v[i][0] = 1;
        v[i][i] = 1;
    }
    for k in 1..size1 {
        for j in 1..k {
            v[k][j] = v[k - 1][j - 1] + v[k - 1][j];
        }
    }
}

fn abs_val(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

fn min(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = input.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
