use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

#[derive(Copy, Clone, PartialEq, Eq)]
enum UnitType {
    Vertical,
    Horizontal,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a = it.next().unwrap().as_bytes();
    let b = it.next().unwrap().as_bytes();

    if n == 0 {
        // No units => multiplicative identity.
        println!("1");
        return;
    }

    // Determine unit sequence by scanning left-to-right.
    // A position i is vertical iff a[i] == b[i], otherwise it must be the left half of a horizontal unit.
    let mut i = 0usize;
    let mut ans: u64;
    let mut prev: UnitType;

    if a[0] == b[0] {
        ans = 3;
        prev = UnitType::Vertical;
        i = 1;
    } else {
        ans = 6;
        prev = UnitType::Horizontal;
        i = 2;
    }
    ans %= MOD;

    while i < n {
        let cur: UnitType;
        if a[i] == b[i] {
            cur = UnitType::Vertical;
            // transition factor
            let factor = match prev {
                UnitType::Vertical => 2,   // V after V
                UnitType::Horizontal => 1, // V after H
            };
            ans = (ans * factor) % MOD;
            prev = cur;
            i += 1;
        } else {
            cur = UnitType::Horizontal;
            // transition factor
            let factor = match prev {
                UnitType::Vertical => 2,   // H after V
                UnitType::Horizontal => 3, // H after H
            };
            ans = (ans * factor) % MOD;
            prev = cur;
            i += 2;
        }
    }

    println!("{}", ans % MOD);
}
