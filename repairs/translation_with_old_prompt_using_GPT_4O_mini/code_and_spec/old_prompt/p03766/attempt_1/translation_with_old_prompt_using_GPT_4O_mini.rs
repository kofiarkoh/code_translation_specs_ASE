use std::io::{self, Write};

const MOD: i32 = 1_000_000_007;

struct BIT {
    data: Vec<i32>,
}

impl BIT {
    fn new(size: usize) -> Self {
        BIT {
            data: vec![0; size + 1],
        }
    }

    fn add(&mut self, n: usize, i: usize, k: i32) {
        let mut idx = i;
        while idx <= n {
            self.data[idx] = (self.data[idx] + k) % MOD;
            idx += idx & (!idx + 1); // Equivalent to idx += (idx & -idx)
        }
    }

    fn sum(&self, r: usize) -> i32 {
        let mut sum = 0;
        let mut idx = r;
        while idx > 0 {
            sum = (sum + self.data[idx]) % MOD;
            idx -= idx & (!idx + 1); // Equivalent to idx -= (idx & -idx)
        }
        sum
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    if n == 1 {
        println!("1");
        return;
    }

    let mut bit = BIT::new(n);
    let mut tmp = (n as i32 * n as i32) % MOD;
    bit.add(n, 1, n as i32);
    bit.add(n, 2, tmp);

    for i in 3..=n {
        tmp = (tmp + bit.sum(i - 3) + (i as i32 - 3) * (n as i32 - 1) + (n as i32 - i as i32 + 2) * n as i32) % MOD;
        bit.add(n, i, tmp);
    }

    println!("{}", tmp);
}
