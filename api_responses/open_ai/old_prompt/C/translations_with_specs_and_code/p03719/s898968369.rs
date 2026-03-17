use std::io::{self, Write};

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
    for &value in &a[1..] {
        if d == 1 {
            break;
        }
        d = gcd(value, d);
    }
    d
}

fn quicksort(a: &mut [char], first: usize, last: usize) {
    let mut i = first;
    let mut j = last;
    let x = a[(first + last) / 2];

    loop {
        while a[i] < x {
            i += 1;
        }
        while x < a[j] {
            j = j.wrapping_sub(1);
        }
        if i >= j {
            break;
        }
        a.swap(i, j);
        i += 1;
        j = j.wrapping_sub(1);
    }

    if first < i.wrapping_sub(1) {
        quicksort(a, first, i.wrapping_sub(1));
    }
    if j.wrapping_add(1) < last {
        quicksort(a, j.wrapping_add(1), last);
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

fn factorize(n: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut a = 2;

    while num >= a {
        if num % a == 0 {
            factors.push(a);
            num /= a;
        } else {
            a += 1;
        }
    }
    factors
}

fn nextperm(p: &mut [i32]) -> i32 {
    let n = p.len();
    let mut i = n - 2;

    while i > 0 && p[i] >= p[i + 1] {
        i -= 1;
    }
    if i == 0 {
        return 0; // Completed
    }

    let mut j = n - 1;
    while p[i] >= p[j] {
        j -= 1;
    }
    p.swap(i, j);
    p[i + 1..].reverse();
    1 // Not completed
}

fn comb(size1: usize, size2: usize) -> Vec<Vec<i32>> {
    let mut v = vec![vec![0; size2]; size1];
    for i in 0..size1 {
        v[i][0] = 1;
        if i < size2 {
            v[i][i] = 1;
        }
    }
    for k in 1..size1 {
        for j in 1..k {
            v[k][j] = v[k - 1][j - 1] + v[k - 1][j];
        }
    }
    v
}

fn abs_val(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

fn min(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (a, b, c) = (nums[0], nums[1], nums[2]);

    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
