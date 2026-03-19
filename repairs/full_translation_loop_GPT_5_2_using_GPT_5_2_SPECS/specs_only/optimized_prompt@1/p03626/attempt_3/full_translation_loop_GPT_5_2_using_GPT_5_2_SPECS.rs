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
    let s1 = it.next().unwrap().as_bytes();
    let s2 = it.next().unwrap().as_bytes();

    if n == 0 {
        println!("1");
        return;
    }

    // The unit decomposition is determined as:
    // - vertical at i if s1[i] == s2[i]
    // - otherwise horizontal covering i and i+1 (per precondition, this is valid)
    //
    // The multiplicative factors follow the standard transition table, but the
    // initial value depends on the first unit:
    // - first vertical: 3
    // - first horizontal: 6
    //
    // For the known target behavior, the transition factors are:
    // - V after V: 2
    // - H after V: 2
    // - V after H: 1
    // - H after H: 3

    let mut i = 0usize;
    let mut ans: u64;
    let mut prev: UnitType;

    if s1[0] == s2[0] {
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
        let cur = if s1[i] == s2[i] {
            UnitType::Vertical
        } else {
            UnitType::Horizontal
        };

        let factor: u64 = match (prev, cur) {
            (UnitType::Vertical, UnitType::Vertical) => 2,
            (UnitType::Vertical, UnitType::Horizontal) => 2,
            (UnitType::Horizontal, UnitType::Vertical) => 1,
            (UnitType::Horizontal, UnitType::Horizontal) => 3,
        };

        ans = (ans * factor) % MOD;
        prev = cur;

        i += match cur {
            UnitType::Vertical => 1,
            UnitType::Horizontal => 2,
        };
    }

    println!("{}", ans);
}
