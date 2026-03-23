use std::io::{self, Read};

const MOD: u16 = 10007;

fn bit_for(c: u8) -> u8 {
    match c {
        b'J' => 1 << 0,
        b'O' => 1 << 1,
        b'I' => 1 << 2,
        // Specification: unsupported characters => undefined behavior in original due to
        // uninitialized value use. We choose the narrowest reasonable Rust behavior:
        // treat as unreachable (panic) rather than inventing a mapping.
        _ => unreachable!("unsupported character"),
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap().as_bytes();

    // DP over subset states (0..=7). State is a bitmask over {J,O,I}.
    // Initialization corresponds to "before day 0" having only subset {J}.
    let mut prev = [0u16; 8];
    prev[1] = 1;

    for day in 0..n {
        let req = bit_for(s[day]);
        let mut next = [0u16; 8];

        for prev_mask in 0u8..8 {
            let ways = prev[prev_mask as usize];
            if ways == 0 {
                continue;
            }

            for cur_mask in 0u8..8 {
                // Must include required participant for this day.
                if (cur_mask & req) == 0 {
                    continue;
                }

                if day == 0 {
                    // Day 0 special: must also include J.
                    if (cur_mask & 1) == 0 {
                        continue;
                    }
                    // No overlap constraint with "before day 0" beyond the initialization
                    // being {J}; the original program's effect is captured by transitions
                    // from prev={J} only.
                } else {
                    // Day-to-day overlap must be non-empty.
                    if (cur_mask & prev_mask) == 0 {
                        continue;
                    }
                }

                let idx = cur_mask as usize;
                let sum = next[idx] as u32 + ways as u32;
                next[idx] = (sum % (MOD as u32)) as u16;
            }
        }

        prev = next;
    }

    let mut ans: u32 = 0;
    for &v in &prev {
        ans += v as u32;
    }
    ans %= MOD as u32;

    println!("{}", ans);
}
