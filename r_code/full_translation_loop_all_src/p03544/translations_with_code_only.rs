use std::io::{self, Read};

type Ll = i64;
type KeyType = i8;

fn gcd(a: i32, b: i32) -> i32 {
    // Mirrors C recursion: return (a % b) ? gcd(b, a % b) : b;
    // Note: In C, calling gcd with b == 0 would be UB (a % 0).
    // We keep the same behavior by not special-casing b == 0.
    if a % b != 0 {
        gcd(b, a % b)
    } else {
        b
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

fn ngcd(n: usize, a: &[i32]) -> i32 {
    let mut d = a[0];
    let mut i = 1usize;
    while i < n && d != 1 {
        d = gcd(a[i], d);
        i += 1;
    }
    d
}

fn quicksort(a: &mut [KeyType], first: isize, last: isize) {
    let x = a[((first + last) / 2) as usize];
    let mut i = first;
    let mut j = last;
    loop {
        while a[i as usize] < x {
            i += 1;
        }
        while x < a[j as usize] {
            j -= 1;
        }
        if i >= j {
            break;
        }
        a.swap(i as usize, j as usize);
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
    let mut i = 2i32;
    while i * i <= x {
        if x % i == 0 {
            return 0;
        }
        i += 1;
    }
    1
}

fn c_strlen_until_nul(s: &[u8]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

fn is_substr(s: &[u8], t: &[u8], mut i: usize) -> i32 {
    let mut flag = 1i32;

    if i >= s.len() || s[i] == 0 {
        flag = 0;
    }

    let tlen = c_strlen_until_nul(t);
    for j in 0..tlen {
        // In C, out-of-bounds would be UB; we avoid panics by treating missing as mismatch.
        let sj = s.get(i).copied().unwrap_or(0);
        if sj != t[j] {
            flag = 0;
            break;
        }
        i += 1;
    }
    flag
}

// factor list, number of factors, natural number
fn factorize(ftr: &mut [i32], num: &mut i32, mut n: i32) {
    let mut a = 2i32;
    let mut cnt = 0i32;

    while n >= a {
        if n % a == 0 {
            n /= a;
            ftr[cnt as usize] = a;
            cnt += 1;
        } else {
            a += 1;
        }
    }
    *num = cnt;
}

// next permutation in lexicographic order
fn nextperm(p: &mut [i32], n: usize) -> i32 {
    let mut i: isize = (n as isize) - 1;
    p[0] = 0; // sentinel
    while p[i as usize] >= p[(i + 1) as usize] {
        i -= 1;
    }
    if i == 0 {
        return 0; // done
    }
    let mut j: isize = n as isize;
    while p[i as usize] >= p[j as usize] {
        j -= 1;
    }
    p.swap(i as usize, j as usize);
    i += 1;
    j = n as isize;
    while i < j {
        p.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
    1 // not done
}

fn comb(size1: usize, size2: usize, v: &mut Vec<Vec<i32>>) {
    // Expects v to be size1 x size2.
    for i in 0..size1 {
        v[i][0] = 1;
        if i < size2 {
            v[i][i] = 1;
        }
    }
    for k in 1..size1 {
        for j in 1..k {
            if j < size2 {
                v[k][j] = v[k - 1][j - 1] + v[k - 1][j];
            }
        }
    }
}

fn abs_val(a: i32, b: i32) -> i32 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

const INF: i32 = 1012345678;

fn main() {
    // Read entire stdin, parse first integer like scanf("%d", &n)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: usize = match it.next() {
        Some(tok) => tok.parse::<i32>().unwrap() as usize,
        None => return,
    };

    let mut l = [0i64; 100];
    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }
    print!("{}\n", l[n]);
}
