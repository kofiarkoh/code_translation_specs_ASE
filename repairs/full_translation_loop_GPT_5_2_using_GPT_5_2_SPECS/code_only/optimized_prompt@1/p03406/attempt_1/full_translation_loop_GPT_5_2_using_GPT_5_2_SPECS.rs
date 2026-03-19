use std::io::{self, Read};

const N: usize = 16;
const M: usize = 16;
const N_: usize = 1 << N;
const MD: i64 = 1_000_000_007;

static mut VV: [i64; N_] = [0; N_];
static mut FF: [i64; N_] = [0; N_];
static mut GG: [i64; N_] = [0; N_];

fn init() {
    // Mirrors the C precomputation exactly, using i64 for intermediate products.
    unsafe {
        FF[0] = 1;
        GG[0] = 1;
        for i in 1..N_ {
            let ii = i as i64;
            // vv[i] = i == 1 ? 1 : vv[i - MD % i] * (MD / i + 1) % MD;
            VV[i] = if i == 1 {
                1
            } else {
                let md_mod_i = (MD % ii) as usize;
                let idx = i - md_mod_i;
                (VV[idx] * (MD / ii + 1)) % MD
            };
            // ff[i] = ff[i - 1] * i % MD;
            FF[i] = (FF[i - 1] * ii) % MD;
            // gg[i] = gg[i - 1] * vv[i] % MD;
            GG[i] = (GG[i - 1] * VV[i]) % MD;
        }
    }
}

fn choose(n: i64, k: i64) -> i64 {
    // In C, choose takes int n,k; if k>n returns 0, otherwise indexes ff/gg.
    // Here we preserve behavior for negative n by returning 0 (avoids invalid indexing),
    // which matches the intended combinatorial usage in the given code.
    if k > n || n < 0 || k < 0 {
        return 0;
    }
    let nu = n as usize;
    let ku = k as usize;
    unsafe { (((FF[nu] * GG[ku]) % MD) * GG[nu - ku]) % MD }
}

fn main() {
    init();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut aa = [0i64; M];
    for h in 0..m {
        let v: i64 = it.next().unwrap().parse().unwrap();
        aa[h] = ((1i64 << n) - v);
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
                    let add = (x * choose(aa[hh] - b as i64, ((1i64 << i) - 1))) % MD;
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
                x = (x * choose(n_, (1i64 << i))) % MD;
                n_ -= 1i64 << i;
                cnt += 1;
            }
        }
        let sign = if cnt % 2 == 0 { 1i64 } else { -1i64 };
        ans = (ans + sign * x) % MD;
    }

    for i in 0..n {
        let fi = unsafe { FF[1usize << i] };
        ans = (((ans * fi) % MD) * 2) % MD;
    }

    if ans < 0 {
        ans += MD;
    }
    print!("{}\n", ans);
}
