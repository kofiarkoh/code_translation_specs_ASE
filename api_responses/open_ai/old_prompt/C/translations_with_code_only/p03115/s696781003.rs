// set many funcs template
// Ver.20181228
use std::io::{self, BufRead};
use std::cmp::{max, min};

const INF: i32 = 1072114514;
const LLINF: i64 = 4154118101919364364;
const MOD: i32 = 1000000007;
const PI: f64 = 3.1415926535897932384;
const CVAL: i64 = 200000000000000000;

fn zt(a: i32, b: i32) -> i32 {
    max(a, b) - min(a, b)
}

fn round(a: i32, b: i32) -> i32 {
    if (a % b) * 2 >= b {
        return (a / b) + 1;
    }
    a / b
}

fn ceil(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        return a / b;
    }
    (a / b) + 1
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    let c = gcd(a, b);
    a / c * b
}

fn ncr(a: i32, b: i32) -> i32 {
    let mut r = 1;
    for i in 1..=b {
        r *= (a + 1 - i);
        r /= i;
    }
    r
}

fn nhr(a: i32, b: i32) -> i32 {
    ncr(a + b - 1, b)
}

fn fact(a: i32) -> i32 {
    let mut r = 1;
    for i in 1..=a {
        r *= i;
    }
    r
}

fn pow(a: i32, b: i32) -> i32 {
    let mut r = 1;
    for _ in 1..=b {
        r *= a;
    }
    r
}

fn dsum(mut x: i32) -> i32 {
    let mut r = 0;
    while x != 0 {
        r += x % 10;
        x /= 10;
    }
    r
}

fn dsumb(mut x: i32, b: i32) -> i32 {
    let mut r = 0;
    while x != 0 {
        r += x % b;
        x /= b;
    }
    r
}

fn sankaku(x: i32) -> i32 {
    ((1 + x) * x) / 2
}

fn llmax(a: i64, b: i64) -> i64 {
    max(a, b)
}

fn llmin(a: i64, b: i64) -> i64 {
    min(a, b)
}

fn llzt(a: i64, b: i64) -> i64 {
    llmax(a, b) - llmin(a, b)
}

fn llround(a: i64, b: i64) -> i64 {
    if (a % b) * 2 >= b {
        return (a / b) + 1;
    }
    a / b
}

fn llceil(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        return a / b;
    }
    (a / b) + 1
}

fn llgcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn lllcm(a: i64, b: i64) -> i64 {
    let c = llgcd(a, b);
    a / c * b
}

fn llncr(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for i in 1..=b {
        r *= (a + 1 - i);
        r /= i;
    }
    r
}

fn llnhr(a: i64, b: i64) -> i64 {
    llncr(a + b - 1, b)
}

fn llfact(a: i64) -> i64 {
    let mut r = 1;
    for i in 1..=a {
        r *= i;
    }
    r
}

fn llpow(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for _ in 1..=b {
        r *= a;
    }
    r
}

fn lldsum(mut x: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % 10;
        x /= 10;
    }
    r
}

fn lldsumb(mut x: i64, b: i64) -> i64 {
    let mut r = 0;
    while x != 0 {
        r += x % b;
        x /= b;
    }
    r
}

fn llsankaku(x: i64) -> i64 {
    ((1 + x) * x) / 2
}

fn dbmax(a: f64, b: f64) -> f64 {
    max(a, b)
}

fn dbmin(a: f64, b: f64) -> f64 {
    min(a, b)
}

fn dbzt(a: f64, b: f64) -> f64 {
    dbmax(a, b) - dbmin(a, b)
}

fn prarr(arr: &[i32]) {
    let output = arr.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
}

fn shuffledget(x: &mut [i32], n: usize) {
    let mut rng = rand::thread_rng();
    let mut b: Vec<usize> = (0..n).collect();
    for i in (1..=n).rev() {
        let p = rand::random::<usize>() % i;
        b.swap(i - 1, p);
    }
    for i in 0..n {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        x[b[i]] = input.trim().parse().unwrap();
    }
}

fn search(x: i32, a: &[i32], n: usize) -> usize {
    let mut st = 0;
    let mut fi = n as isize - 1;
    while st <= fi {
        let te = (st + fi) / 2;
        if a[te as usize] < x {
            st = te + 1;
        } else {
            fi = te - 1;
        }
    }
    st as usize
}

fn ncrbit(n: i64, r: i64) -> i64 {
    if n < 0 || r < 0 || n < r || (n | r) != n {
        return 0;
    }
    1
}

fn findcol(a: i64, b: i64, x: &[i64], y: &[i64], n: usize) -> i64 {
    let mut r = 0;
    for i in 0..n {
        if a - x[i] < 0 || b - y[i] < 0 {
            continue;
        }
        let p = (a - x[i]) + (b - y[i]);
        let q = (b - y[i]);
        r ^= ncrbit(p, q);
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let nums: Vec<i64> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = nums[0];
        y[i] = nums[1];
    }

    let mut r = 0;
    for k in (0..=60).rev() {
        let w = 1 << k;
        if findcol(CVAL - (r + w), r + w, &x, &y, n) == 1 {
            r += w;
        }
    }
    println!("{}", CVAL - r);
}
