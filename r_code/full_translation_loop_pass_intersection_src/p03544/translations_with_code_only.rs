use std::io::{self, Read};

type Ll = i64;
type KeyType = i8;

fn gcd(a: i32, b: i32) -> i32 {
    // Mirrors the C recursion: return (a % b) ? gcd(b, a % b) : b;
    // Note: In C, calling with b == 0 would be UB (a % 0). We keep the same
    // precondition; the provided program never triggers it.
    if a % b != 0 {
        gcd(b, a % b)
    } else {
        b
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
}

/* a[0], ..., a[n-1] の数の最大公約数 */
fn ngcd(n: i32, a: &[i32]) -> i32 {
    let mut d = a[0];
    let mut i = 1;
    while i < n && d != 1 {
        d = gcd(a[i as usize], d);
        i += 1;
    }
    d
}

fn quicksort(a: &mut [KeyType], first: i32, last: i32) {
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
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return 0;
        }
        i += 1;
    }
    1
}

fn c_strlen(s: &[u8]) -> usize {
    s.iter().position(|&c| c == 0).unwrap_or(s.len())
}

fn is_substr(s: &[u8], t: &[u8], mut i: usize) -> i32 {
    let mut flag = 1;
    if i >= s.len() || s[i] == 0 {
        flag = 0;
    }

    // C code calls strlen(t) each iteration; we compute it once (same result).
    let tlen = c_strlen(t);
    for j in 0..tlen {
        if i >= s.len() || s[i] != t[j] {
            flag = 0;
            break;
        }
        i += 1;
    }
    flag
}

// factor list, number of factors, natural number
fn factorize(ftr: &mut [i32], num: &mut i32, mut n: i32) {
    let mut a = 2;
    let mut cnt = 0;
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

// int nextperm(int p[], int N)  /* 辞書式順序で次の順列 */
fn nextperm(p: &mut [i32], n: i32) -> i32 {
    let mut i = n - 1;
    p[0] = 0; // 番人
    while p[i as usize] >= p[(i + 1) as usize] {
        i -= 1;
    }
    if i == 0 {
        return 0; // 完了
    }
    let mut j = n;
    while p[i as usize] >= p[j as usize] {
        j -= 1;
    }
    p.swap(i as usize, j as usize);
    i += 1;
    j = n;
    while i < j {
        p.swap(i as usize, j as usize);
        i += 1;
        j -= 1;
    }
    1 // 未了
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut l = [0i64; 100];

    // #if 1 block
    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 2] + l[i - 1];
    }
    print!("{}\n", l[n]);
}
