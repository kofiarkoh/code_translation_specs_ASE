use std::io::{self, Read};

const N: usize = 16;
const M: usize = 16;
const N_: usize = 1 << N;
const MD: i64 = 1_000_000_007;

static mut VV: [i64; N_] = [0; N_];
static mut FF: [i64; N_] = [0; N_];
static mut GG: [i64; N_] = [0; N_];

fn init() {
    // Uses global arrays sized for N_=2^16; safe because indices are bounded by loops.
    unsafe {
        FF[0] = 1;
        GG[0] = 1;
        for i in 1..N_ {
            let ii = i as i64;
            // vv[i] computes modular inverse of i mod MD (with vv[1]=1 special-cased),
            // using the classic recurrence:
            // inv[i] = inv[MD % i] * (MD - MD/i) mod MD
            // which is equivalent to inv[i] = inv[i - MD%i] * (MD/i + 1) mod MD
            // because i - (MD % i) == (MD / i) * i.
            VV[i] = if i == 1 {
                1
            } else {
                let r = (MD % ii) as usize;
                let idx = i - r; // i - (MD % i)
                (VV[idx] * (MD / ii + 1)) % MD
            };
            FF[i] = (FF[i - 1] * ii) % MD;
            GG[i] = (GG[i - 1] * VV[i]) % MD;
        }
    }
}

fn choose(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    // n,k are expected to be within [0, N_) in this program; index as usize.
    let nu = n as usize;
    let ku = k as usize;
    unsafe { (((FF[nu] * GG[ku]) % MD) * GG[nu - ku]) % MD }
}

fn main() {
    init();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut aa = [0i64; M];
    for h in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        aa[h] = ((1i64 << n) - v) as i64;
    }

    let size = 1usize << n;
    let mut dp = vec![0i64; size];
    dp[0] = 1;

    for hh in (0..m).rev() {
        for b in (0..size).rev() {
            let x = dp[b];
            if x == 0 {
                continue;
            }
            for i in 0..n {
                if (b & (1usize << i)) == 0 {
                    let nb = b | (1usize << i);
                    let add = (x * choose(aa[hh] - b as i64, (1i64 << i) - 1)) % MD;
                    dp[nb] = (dp[nb] + add) % MD;
                }
            }
        }
    }

    let mut ans: i64 = 0;
    for b in 0..size {
        let mut x = dp[b];
        if x == 0 {
            continue;
        }
        let mut n_ = (size as i64) - 1 - b as i64;
        let mut cnt: i64 = n as i64;
        for i in 0..n {
            if (b & (1usize << i)) == 0 {
                x = (x * choose(n_, 1i64 << i)) % MD;
                n_ -= 1i64 << i;
                cnt += 1;
            }
        }
        let sign = if cnt % 2 == 0 { 1 } else { -1 };
        ans = (ans + sign * x) % MD;
    }

    unsafe {
        for i in 0..n {
            ans = (((ans * FF[1usize << i]) % MD) * 2) % MD;
        }
    }

    if ans < 0 {
        ans += MD;
    }
    print!("{}\n", ans);
}
