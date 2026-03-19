use std::io::{self, Read};

type Ll = i64;
type KeyType = i8;

fn gcd(a: i32, b: i32) -> i32 {
    // Mirrors C recursion: return (a % b) ? gcd(b, a % b) : b;
    // Note: In C, calling with b == 0 is UB (a % 0). We keep the same assumption.
    if a % b != 0 {
        gcd(b, a % b)
    } else {
        b
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    // In C, this can overflow (UB). Rust i32 overflow in release wraps, in debug panics.
    // Use wrapping ops to better emulate typical C behavior on two's complement machines.
    a.wrapping_div(gcd(a, b)).wrapping_mul(b)
}

/* a[0], ..., a[n-1] の数の最大公約数 */
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
    while i.wrapping_mul(i) <= x {
        if x % i == 0 {
            return 0;
        }
        i += 1;
    }
    1
}

fn is_substr(s: &[u8], t: &[u8], mut i: usize) -> i32 {
    // C version expects NUL-terminated strings and uses strlen(t) each loop.
    // Here we treat inputs as byte slices; caller must provide NUL-terminated `s`
    // and `t` without the trailing NUL (or include it consistently).
    let mut flag = 1i32;

    if i >= s.len() || s[i] == b'\0' {
        flag = 0;
    }

    // Emulate `for (j = 0; j < strlen(t); ++j, ++i)`
    let tlen = t.len();
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

// int nextperm(int p[], int N)  /* 辞書式順序で次の順列 */
fn nextperm(p: &mut [i32], n: usize) -> i32 {
    // C uses p[0] as sentinel and accesses p[i+1] up to p[N].
    // Caller must provide p with length at least N+1.
    let mut i: isize = (n as isize) - 1;
    p[0] = 0; // sentinel
    while p[i as usize] >= p[(i + 1) as usize] {
        i -= 1;
    }
    if i == 0 {
        return 0; // complete
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
    1 // not complete
}

fn comb(size1: usize, size2: usize, v: &mut Vec<Vec<i32>>) {
    // Mirrors the C logic; assumes v is size1 x size2.
    for i in 0..size1 {
        v[i][0] = 1;
        if i < size2 {
            v[i][i] = 1;
        }
    }
    for k in 1..size1 {
        for j in 1..k {
            if j < size2 && k - 1 < size1 {
                v[k][j] = v[k - 1][j - 1].wrapping_add(v[k - 1][j]);
            }
        }
    }
}

fn abs_val(a: i32, b: i32) -> i32 {
    if a > b {
        a.wrapping_sub(b)
    } else {
        b.wrapping_sub(a)
    }
}

fn min_i32(a: i32, b: i32) -> i32 {
    if a > b { b } else { a }
}

const INF: i32 = 1012345678;

fn main() {
    // Read all input, parse three ints like scanf("%d %d %d", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    if a <= c && c <= b {
        print!("Yes\n");
    } else {
        print!("No\n");
    }
}
