// thx Ebi-chan!

const BIG: u64 = 2000000007;
const VERYBIG: u64 = 2000000000000000007;
const ULTRABIG: u64 = 300000000000000000000;

const MOD: u64 = 1000000007;
const FOD: u64 = 998244353;
const HOD: u64 = 1000000009;
const IOD: u64 = 100000007;

type Ull = u64;
type Sll = i64;

const N_MAX: usize = 1048576;

#[derive(Clone, Copy)]
struct Hwll {
    a: Sll,
    b: Sll,
}

#[derive(Clone, Copy)]
struct Hwllc {
    a: Sll,
    b: Sll,
    c: Sll,
}

#[derive(Clone, Copy)]
struct Linell {
    a: Hwll,
    b: Hwll,
}

#[derive(Clone, Copy)]
struct Hwreal {
    a: f64,
    b: f64,
}

static mut N: Sll = 0;
static mut M: Sll = 0;

fn umin(x: Ull, y: Ull) -> Ull {
    if x < y { x } else { y }
}

fn umax(x: Ull, y: Ull) -> Ull {
    if x > y { x } else { y }
}

fn smin(x: Sll, y: Sll) -> Sll {
    if x < y { x } else { y }
}

fn smax(x: Sll, y: Sll) -> Sll {
    if x > y { x } else { y }
}

fn dmin(x: f64, y: f64) -> f64 {
    if x < y { x } else { y }
}

fn dmax(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

fn gcd(x: Ull, y: Ull) -> Ull {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn bitpow(a: Ull, mut x: Ull, modulo: Ull) -> Ull {
    let mut result = 1;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * a) % modulo;
        }
        x /= 2;
        a = (a * a) % modulo;
    }
    result
}

fn divide(a: Ull, b: Ull, modulo: Ull) -> Ull {
    (a * bitpow(b, modulo - 2, modulo)) % modulo
}

fn udiff(a: Ull, b: Ull) -> Ull {
    if a >= b { a - b } else { b - a }
}

fn sdiff(a: Sll, b: Sll) -> Sll {
    if a >= b { a - b } else { b - a }
}

fn bitcount(mut n: Ull) -> i32 {
    let mut result = 0;
    while n > 0 {
        if n & 1 == 1 {
            result += 1;
        }
        n /= 2;
    }
    result
}

fn pstrcomp(left: &*const *const i8, right: &*const *const i8) -> i32 {
    let l = unsafe { std::ffi::CStr::from_ptr(**left).to_str().unwrap() };
    let r = unsafe { std::ffi::CStr::from_ptr(**right).to_str().unwrap() };
    l.cmp(r) as i32
}

fn isinrange(left: Sll, x: Sll, right: Sll) -> bool {
    left <= x && x <= right
}

fn isinrange_soft(left: Sll, x: Sll, right: Sll) -> bool {
    (left <= x && x <= right) || (left >= x && x >= right)
}

fn sw(l: &mut Sll, r: &mut Sll) {
    if *l == *r { return; }
    let t = *l;
    *l = *r;
    *r = t;
}

fn uw(l: &mut Ull, r: &mut Ull) {
    if *l == *r { return; }
    let t = *l;
    *l = *r;
    *r = t;
}

static mut FRAC: [Ull; N_MAX * 10] = [0; N_MAX * 10];
static mut INVF: [Ull; N_MAX * 10] = [0; N_MAX * 10];

fn f_init(n: Sll, m: Sll) {
    unsafe {
        FRAC[0] = 1;
        for i in 1..=n {
            FRAC[i as usize] = (FRAC[(i - 1) as usize] * i) % m;
        }

        INVF[n as usize] = divide(1, FRAC[n as usize], m);
        for i in (0..n).rev() {
            INVF[i as usize] = (INVF[(i + 1) as usize] * (i + 1)) % m;
        }
    }
}

fn ncr(n: Sll, r: Sll, m: Ull) -> Ull {
    if n < 0 || r < 0 || n < r { return 0; }
    unsafe {
        (FRAC[n as usize] * (INVF[r as usize] * INVF[(n - r) as usize] % m)) % m
    }
}

static mut A: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut B: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut C: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut D: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut E: [Sll; N_MAX * 4] = [0; N_MAX * 4];
static mut S: [i8; N_MAX + 1] = [0; N_MAX + 1];
static mut T: [i8; N_MAX + 1] = [0; N_MAX + 1];
static mut U: [i8; N_MAX + 1] = [0; N_MAX + 1];

static mut DP: [[Sll; 5050]; 5050] = [[0; 5050]; 5050];
static mut DQ: [[Sll; 5050]; 5050] = [[0; 5050]; 5050];

static mut POW2: [Sll; N_MAX] = [0; N_MAX];
static mut INV2: [Sll; N_MAX] = [0; N_MAX];

fn solve() -> Sll {
    unsafe {
        let inv = (MOD + 1) / 2;

        POW2[0] = 1;
        INV2[0] = 1;
        for i in 1..=N + M {
            POW2[i as usize] = (POW2[(i - 1) as usize] * 2) % MOD;
            INV2[i as usize] = (INV2[(i - 1) as usize] * inv) % MOD;
        }

        let mut result = 0;

        for i in 0..=N {
            for j in 1..=M {
                if i == 0 {
                    DP[i as usize][j as usize] = 1;
                } else {
                    DP[i as usize][j as usize] = ((j + 1) * DP[(i - 1) as usize][j as usize] + DQ[(i - 1) as usize][(j - 1) as usize] * POW2[(j - 1) as usize]) % MOD;
                }

                DQ[i as usize][j as usize] = (DQ[i as usize][(j - 1) as usize] + (DP[i as usize][j as usize] * j % MOD) * INV2[j as usize]) % MOD;
            }
        }
        result = DP[N as usize][M as usize];

        println!("{}", result);
        return 0;
    }
}

fn main() {
    unsafe {
        let mut i: Sll;
        let mut j: Sll;

        N = 3;
        M = 0;

        // Simulating input
        std::io::stdin().read_line(&mut String::new()).unwrap();
        std::io::stdin().read_line(&mut String::new()).unwrap();
        std::io::stdin().read_line(&mut String::new()).unwrap();

        for i in 0..0 {
            // Simulating input
            std::io::stdin().read_line(&mut String::new()).unwrap();
            A[i as usize] -= 1;
        }

        solve();
    }
}
