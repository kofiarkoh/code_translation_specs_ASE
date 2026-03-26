use std::io;

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
        (FF[n as usize] * GG[k as usize] % MD * GG[(n - k) as usize]) % MD
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut aa = vec![0; m];
    for h in 0..m {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        aa[h] = (1 << n) - input.trim().parse::<i64>().unwrap();
    }

    let mut dp = vec![0; N_];
    unsafe {
        dp[0] = 1;
    }

    for h in (0..m).rev() {
        for b in (0..(1 << n)).rev() {
            let x = unsafe { dp[b] };
            if x == 0 {
                continue;
            }
            for i in 0..n {
                if (b & (1 << i)) == 0 {
                    let new_b = b | (1 << i);
                    unsafe {
                        let choose_value = choose(aa[h] - b as i64, (1 << i) as i64 - 1);
                        dp[new_b] = (dp[new_b] + x * choose_value) % MD;
                    }
                }
            }
        }
    }

    let mut ans = 0;
    for b in 0..(1 << n) {
        let mut x = unsafe { dp[b] };
        if x == 0 {
            continue;
        }
        let mut n_ = (1 << n) - 1 - b;
        let mut cnt = n;
        for i in 0..n {
            if (b & (1 << i)) == 0 {
                x = x * choose(n_ as i64, (1 << i) as i64) % MD;
                n_ -= 1 << i;
                cnt += 1;
            }
        }
        ans = (ans + if cnt % 2 == 0 { x } else { -x }) % MD;
    }

    for i in 0..n {
        ans = (ans * unsafe { FF[1 << i] } % MD * 2) % MD;
    }
    if ans < 0 {
        ans += MD;
    }
    println!("{}", ans);
}
