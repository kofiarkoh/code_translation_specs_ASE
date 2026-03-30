use std::io::{self, BufRead};

const N: usize = 16;
const M: usize = 16;
const N_: usize = 1 << N;
const MD: i64 = 1_000_000_007;

struct Globals {
    vv: [i64; N_],
    ff: [i64; N_],
    gg: [i64; N_],
}

impl Globals {
    fn new() -> Self {
        let mut vv = [0; N_];
        let mut ff = [0; N_];
        let mut gg = [0; N_];

        ff[0] = 1;
        gg[0] = 1;

        for i in 1..N_ {
            vv[i] = if i == 1 {
                1
            } else {
                (vv[i - (MD as usize % i)] * (MD / i as i64 + 1)) % MD
            };
            ff[i] = (ff[i - 1] * i as i64) % MD;
            gg[i] = (gg[i - 1] * vv[i]) % MD;
        }

        Globals { vv, ff, gg }
    }

    fn choose(&self, n: usize, k: usize) -> i64 {
        if k > n {
            return 0;
        }
        (self.ff[n] * self.gg[k] % MD * self.gg[n - k] % MD) % MD
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    let first_line = input.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    let mut aa = [0; M];
    for h in 0..m {
        let line = input.next().unwrap().unwrap();
        aa[h as usize] = (1 << n) - line.parse::<i32>().unwrap();
    }

    let mut dp = [0; N_];
    dp[0] = 1;

    let globals = Globals::new();

    for h in (0..m).rev() {
        for b in (0..(1 << n)).rev() {
            let x = dp[b as usize];

            if x == 0 {
                continue;
            }
            for i in 0..n {
                if (b & (1 << i)) == 0 {
                    dp[(b | (1 << i)) as usize] = (dp[(b | (1 << i)) as usize] + x * globals.choose(aa[h as usize] as usize - b, (1 << i) - 1) % MD) % MD;
                }
            }
        }
    }

    let mut ans = 0;
    for b in 0..(1 << n) {
        let mut x = dp[b as usize];
        if x == 0 {
            continue;
        }
        let mut n_ = (1 << n) - 1 - b;
        let mut cnt = n;

        for i in 0..n {
            if (b & (1 << i)) == 0 {
                x = x * globals.choose(n_, 1 << i) % MD;
                n_ -= 1 << i;
                cnt += 1;
            }
        }
        ans = (ans + if cnt % 2 == 0 { x } else { -x }) % MD;
    }

    for i in 0..n {
        ans = (ans * globals.ff[1 << i] % MD * 2) % MD;
    }
    if ans < 0 {
        ans += MD;
    }
    println!("{}", ans);
}
