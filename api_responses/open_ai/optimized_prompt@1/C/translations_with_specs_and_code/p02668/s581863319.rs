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

fn f_init(n: usize, m: u64, frac: &mut Vec<u64>, invf: &mut Vec<u64>) {
    frac[0] = 1;
    for i in 1..=n {
        frac[i] = (frac[i - 1] * i as u64) % m;
    }

    invf[n] = divide(1, frac[n], m);
    for i in (0..n).rev() {
        invf[i] = (invf[i + 1] * (i + 1) as u64) % m;
    }
}

fn ncr(n: i64, r: i64, frac: &Vec<u64>, invf: &Vec<u64>, m: u64) -> u64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    (frac[n as usize] * (invf[r as usize] * invf[(n - r) as usize] % m)) % m
}

fn solve(n: usize, m: usize) -> i64 {
    let mut dp = vec![vec![0; m + 1]; n + 1];
    let mut dq = vec![vec![0; m + 1]; n + 1];
    let inv = (MOD + 1) / 2;

    let mut pow2 = vec![0; n + m + 1];
    let mut inv2 = vec![0; n + m + 1];

    pow2[0] = 1;
    inv2[0] = 1;
    for i in 1..=n + m {
        pow2[i] = (pow2[i - 1] * 2) % MOD;
        inv2[i] = (inv2[i - 1] * inv) % MOD;
    }

    for i in 0..=n {
        for j in 1..=m {
            if i == 0 {
                dp[i][j] = 1;
            } else {
                dp[i][j] = ((j + 1) * dp[i - 1][j] + dq[i - 1][j - 1] * pow2[j - 1]) % MOD;
            }
            dq[i][j] = (dq[i][j - 1] + (dp[i][j] * j as u64 % MOD) * inv2[j]) % MOD;
        }
    }
    dp[n][m] as i64
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = nums[0] as usize;
    let m = nums[1] as usize;

    if n < 0 || m < 0 {
        println!("-1");
        return;
    }

    let mut frac = vec![0; n + 5];
    let mut invf = vec![0; n + 5];
    f_init(n + 5, MOD, &mut frac, &mut invf);

    let result = solve(n, m);
    if result < 0 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
