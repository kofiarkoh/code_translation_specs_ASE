use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn add_bit(n: usize, bit: &mut [i64], mut i: usize, k: i64) {
    while i <= n {
        bit[i] = (bit[i] + k) % MOD;
        // i += (i & -i) with 1-based indexing
        let lsb = i & (!i + 1);
        i += lsb;
    }
}

fn sum_bit(bit: &[i64], mut r: usize) -> i64 {
    let mut sum: i64 = 0;
    while r > 0 {
        sum = (sum + bit[r]) % MOD;
        let lsb = r & (!r + 1);
        r -= lsb;
    }
    sum
}

fn main() {
    // Read entire stdin, parse first integer like scanf("%d", &n)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n_i32: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let n = n_i32 as usize;

    if n_i32 == 1 {
        print!("1\n");
        // stdout is line-buffered on terminals; explicit flush not required for correctness here.
        return;
    }

    // C: int BIT[1000001] = {};
    // Use i64 to avoid overflow; all operations are modded by MOD.
    let mut bit = vec![0i64; 1_000_001];

    // C: tmp = (int)((long long)n * n % Mod);
    let mut tmp: i64 = ((n_i32 as i64) * (n_i32 as i64)) % MOD;

    add_bit(n, &mut bit, 1, n_i32 as i64);
    add_bit(n, &mut bit, 2, tmp);

    for i in 3..=n {
        // tmp = (tmp + sum_BIT(BIT, i - 3) + (long long)(i - 3) * (n - 1)
        //        + (long long)(n - i + 2) * n) % Mod;
        let i_i64 = i as i64;
        let n_i64 = n as i64;

        let s = sum_bit(&bit, i - 3);
        let term1 = (i_i64 - 3) * (n_i64 - 1);
        let term2 = (n_i64 - i_i64 + 2) * n_i64;

        tmp = (tmp + s + term1 + term2) % MOD;
        add_bit(n, &mut bit, i, tmp);
    }

    print!("{}\n", tmp);
}
