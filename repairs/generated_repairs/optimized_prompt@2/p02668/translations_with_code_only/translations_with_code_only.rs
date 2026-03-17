// thx Ebi-chan!

const BIG: u64 = 2000000007;
const VERYBIG: u64 = 2000000000000000007;
const ULTRABIG: u64 = 300000000000000000000;

const MOD: i64 = 1000000007; // Changed to i64
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
static mut H: Sll = 0;
static mut W: Sll = 0;

static mut A: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut C: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut D: [Sll; N_MAX * 5] = [0; N_MAX * 5];
static mut DP: [[Sll; 5050]; 5050] = [[0; 5050]; 5050];
static mut DQ: [[Sll; 5050]; 5050] = [[0; 5050]; 5050];
static mut POW2: [Sll; N_MAX] = [0; N_MAX];
static mut INV2: [Sll; N_MAX] = [0; N_MAX];

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

fn gcd(x: Ull, y: Ull) -> Ull {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn bitpow(mut a: Ull, mut x: Ull, modulo: Ull) -> Ull {
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

fn solve() -> Sll {
    unsafe {
        let inv = (MOD + 1) / 2;

        POW2[0] = 1;
        INV2[0] = 1;
        for i in 1..=N + M {
            POW2[i as usize] = (POW2[(i - 1) as usize] * 2) % MOD;
            INV2[i as usize] = (INV2[(i - 1) as usize] * inv) % MOD;
        }

        let mut result: Sll = 0;

        for i in 0..=N {
            for j in 1..=M {
                if i == 0 {
                    DP[i as usize][j as usize] = 1;
                } else {
                    DP[i as usize][j as usize] = ((j + 1) * DP[(i - 1) as usize][j as usize] + DQ[(i - 1) as usize][(j - 1) as usize] * POW2[(j - 1) as usize]) % MOD;
                }

                DQ[i as usize][j as usize] = (DQ[i as usize][(j - 1) as usize] + (DP[i as usize][j as usize] * j as Sll % MOD) * INV2[j as usize]) % MOD;
            }
        }
        result = DP[N as usize][M as usize];

        println!("{}", result);
    }
    0
}

fn main() {
    unsafe {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        N = iter.next().unwrap().parse().unwrap();
        M = iter.next().unwrap().parse().unwrap();
        H = 5; // Hardcoded as per original C code

        for i in 0..0 {
            A[i as usize] = iter.next().unwrap().parse().unwrap();
            A[i as usize] -= 1; // Adjusting for 0-based index
        }

        solve();
    }
}
