use std::io::{self, Read};

const MOD: i32 = 10007;

fn char_to_bit(c: u8) -> u8 {
    // The three supported symbols in this problem are 'J', 'O', 'I' (JOI).
    // Map them to the three participants in that order.
    match c {
        b'J' => 0,
        b'O' => 1,
        b'I' => 2,
        _ => 0, // Unspecified behavior for unsupported symbols; deterministic fallback.
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: usize = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => return,
    };
    let s = it.next().unwrap_or("");

    if n == 0 {
        println!("1");
        return;
    }

    let bytes = s.as_bytes();

    // dp[mask] = number of valid sequences up to current day ending with subset == mask (mod MOD)
    let mut dp = [0i32; 8];
    let mut next = [0i32; 8];

    // Fixed initially-required participant is the first of the three participants => bit 0 ('J').
    let initial_bit: u8 = 1 << 0;

    // Day 0: subset must include both initial participant and required participant for day 0.
    let r0_bit = 1u8 << char_to_bit(bytes[0]);
    let must0 = initial_bit | r0_bit;
    for mask in 0u8..8 {
        if (mask & must0) == must0 {
            dp[mask as usize] = 1;
        }
    }

    // Days 1..N-1 transitions.
    for d in 1..n {
        next.fill(0);
        let rd_bit = 1u8 << char_to_bit(bytes[d]);

        for prev in 0u8..8 {
            let prev_count = dp[prev as usize];
            if prev_count == 0 {
                continue;
            }
            for cur in 0u8..8 {
                if (cur & rd_bit) == 0 {
                    continue; // must include required participant
                }
                if (cur & prev) == 0 {
                    continue; // must overlap with previous day
                }
                let idx = cur as usize;
                let mut v = next[idx] + prev_count;
                v %= MOD;
                next[idx] = v;
            }
        }

        dp = next;
    }

    let mut ans: i32 = 0;
    for &v in &dp {
        ans += v;
        ans %= MOD;
    }
    println!("{}", ans);
}
