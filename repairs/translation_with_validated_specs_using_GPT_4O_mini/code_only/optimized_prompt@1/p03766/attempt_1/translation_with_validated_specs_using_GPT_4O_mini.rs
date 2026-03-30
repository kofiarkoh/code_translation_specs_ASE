use std::io::{self, Write};

const MOD: i32 = 1_000_000_007;

fn add_bit(n: usize, bit: &mut Vec<i32>, i: usize, k: i32) {
    let mut idx = i;
    while idx <= n {
        bit[idx] = (bit[idx] + k) % MOD;
        idx += idx & (!idx + 1); // i & -i in C
    }
}

fn sum_bit(bit: &Vec<i32>, r: usize) -> i32 {
    let mut sum = 0;
    let mut idx = r;
    while idx > 0 {
        sum = (sum + bit[idx]) % MOD;
        idx -= idx & (!idx + 1); // r & -r in C
    }
    sum
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n == 1 {
        println!("1");
        io::stdout().flush().unwrap();
        return;
    }

    let mut bit = vec![0; 1_000_001];
    let mut tmp = (n as i64 * n as i64 % MOD as i64) as i32;

    add_bit(n as usize, &mut bit, 1, n);
    add_bit(n as usize, &mut bit, 2, tmp);

    for i in 3..=n {
        tmp = (tmp + sum_bit(&bit, (i - 3) as usize) + (i - 3) as i64 * (n - 1) as i64 + (n - i + 2) as i64 * n as i64) % MOD as i64;
        add_bit(n as usize, &mut bit, i as usize, tmp);
    }

    println!("{}", tmp);
    io::stdout().flush().unwrap();
}
