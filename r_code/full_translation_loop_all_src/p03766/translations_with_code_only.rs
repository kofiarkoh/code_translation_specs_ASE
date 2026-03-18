use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn add_bit(n: usize, bit: &mut [i64], mut i: usize, k: i64) {
    while i <= n {
        bit[i] = (bit[i] + k) % MOD;
        // i += (i & -i) for 1-based BIT indices
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
    let n = n_i32 as i64;

    if n_i32 == 1 {
        print!("1\n");
        return;
    }

    // C code uses BIT[1000001] and indexes 1..=n.
    // We allocate exactly 1_000_001 elements to match.
    let mut bit = vec![0i64; 1_000_001];

    let mut tmp: i64 = (n * n) % MOD;

    add_bit(n_i32 as usize, &mut bit, 1, n % MOD);
    add_bit(n_i32 as usize, &mut bit, 2, tmp);

    for i in 3..=n_i32 {
        let i64v = i as i64;
        let term1 = sum_bit(&bit, (i - 3) as usize);
        let term2 = (i64v - 3) * (n - 1);
        let term3 = (n - i64v + 2) * n;
        tmp = (tmp + term1 + term2 + term3) % MOD;
        add_bit(n_i32 as usize, &mut bit, i as usize, tmp);
    }

    print!("{}\n", tmp);
}
