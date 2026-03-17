// thx Ebi-chan!

const MOD: u64 = 1_000_000_007;

fn bitpow(a: u64, mut x: u64, modulo: u64) -> u64 {
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

fn divide(a: u64, b: u64, modulo: u64) -> u64 {
    (a * bitpow(b, modulo - 2, modulo)) % modulo
}

fn ncr(n: i64, r: i64, m: u64) -> u64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let mut frac = vec![1; (n + 1) as usize];
    let mut invf = vec![0; (n + 1) as usize];

    for i in 1..=n {
        frac[i as usize] = (frac[(i - 1) as usize] * i as u64) % m;
    }

    invf[n as usize] = divide(1, frac[n as usize], m);
    for i in (0..n).rev() {
        invf[i as usize] = (invf[(i + 1) as usize] * (i + 1) as u64) % m;
    }

    frac[n as usize] * (invf[r as usize] * invf[(n - r) as usize] % m) % m
}

fn solve(n: i64, m: i64) -> i64 {
    if n < 0 || m < 0 || n < m {
        return -1;
    }

    let mut dp = vec![vec![0; (m + 1) as usize]; (n + 1) as usize];
    let mut dq = vec![vec![0; (m + 1) as usize]; (n + 1) as usize];

    let inv = (MOD + 1) / 2;
    let mut pow2 = vec![1; (n + m + 1) as usize];
    let mut inv2 = vec![1; (n + m + 1) as usize];

    for i in 1..=n + m {
        pow2[i as usize] = (pow2[(i - 1) as usize] * 2) % MOD;
        inv2[i as usize] = (inv2[(i - 1) as usize] * inv) % MOD;
    }

    for i in 0..=n {
        for j in 1..=m {
            if i == 0 {
                dp[i as usize][j as usize] = 1;
            } else {
                dp[i as usize][j as usize] = ((j + 1) * dp[(i - 1) as usize][j as usize] + dq[(i - 1) as usize][j as usize - 1] * pow2[(j - 1) as usize]) % MOD;
            }
            dq[i as usize][j as usize] = (dq[i as usize][j as usize - 1] + (dp[i as usize][j as usize] * j % MOD) * inv2[j as usize]) % MOD;
        }
    }

    dp[n as usize][m as usize] as i64
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = nums[0];
    let m = nums[1];

    let result = solve(n, m);
    println!("{}", result);
}
