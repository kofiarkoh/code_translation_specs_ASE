use std::io::{self, Read};

const N: usize = 16;
const M: usize = 16;
const N_: usize = 1 << N;
const MD: i64 = 1_000_000_007;

static mut VV: [i64; N_] = [0; N_];
static mut FF: [i64; N_] = [0; N_];
static mut GG: [i64; N_] = [0; N_];

fn init() {
    unsafe {
        FF[0] = 1;
        GG[0] = 1;
        for i in 1..N_ {
            VV[i] = if i == 1 {
                1
            } else {
                (VV[i - (MD % i as i64) as usize] * (MD / i as i64 + 1)) % MD
            };
            FF[i] = (FF[i - 1] * i as i64) % MD;
            GG[i] = (GG[i - 1] * VV[i]) % MD;
        }
    }
}

fn choose(n: i64, k: i64) -> i64 {
    if k > n {
        return 0;
    }
    unsafe {
        (FF[n as usize] * GG[k as usize] % MD * GG[(n - k) as usize] % MD) % MD
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let first_line = lines.next().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: i64 = nums.next().unwrap().parse().unwrap();
    let m: i64 = nums.next().unwrap().parse().unwrap();

    let mut aa = [0; M];
    for h in 0..m {
        let value: i64 = lines.next().unwrap().parse().unwrap();
        aa[h as usize] = (1 << n) - value;
    }

    let mut dp = [0; N_];
    unsafe {
        dp[0] = 1;
    }

    for h in (0..m).rev() {
        for b in (0..(1 << n)).rev() {
            let x = unsafe { dp[b as usize] };

            if x == 0 {
                continue;
            }
            for i in 0..n {
                if (b & (1 << i)) == 0 {
                    let new_b = b | (1 << i);
                    unsafe {
                        dp[new_b as usize] = (dp[new_b as usize] + x * choose(aa[h as usize] - b, (1 << i) - 1)) % MD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for b in 0..(1 << n) {
        let mut x = unsafe { dp[b as usize] };
        if x == 0 {
            continue;
        }
        let mut n_ = (1 << n) - 1 - b;
        let mut cnt = n;

        for i in 0..n {
            if (b & (1 << i)) == 0 {
                x = x * choose(n_, 1 << i) % MD;
                n_ -= 1 << i;
                cnt += 1;
            }
        }
        ans = (ans + if cnt % 2 == 0 { x } else { -x }) % MD;
    }

    for i in 0..n {
        ans = (ans * unsafe { FF[1 << i] } * 2) % MD;
    }
    if ans < 0 {
        ans += MD;
    }
    println!("{}", ans);
}
