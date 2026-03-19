use std::io::{self, Read};

const MOD: i32 = 10007;

fn char_to_bit(c: u8) -> u8 {
    // Narrowest reasonable mapping consistent with 3 distinct keys:
    // represent each key as a distinct bit in a 3-bit mask.
    match c {
        b'J' => 1, // 0b001
        b'O' => 2, // 0b010
        b'I' => 4, // 0b100
        _ => 0,     // Preconditions say this won't happen.
    }
}

fn main() {
    // Read entire stdin; specification provides no exceptional behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let s = it.next().unwrap_or("");

    // Schedule arrays: counts of ways to have each held-keys mask (1..=7) at each step.
    // Fixed size (8) as required; index 0 unused (empty set not a valid "held keys" state).
    let mut prev = [0i32; 8];
    let mut next = [0i32; 8];

    // Process up to N characters; precondition says s has exactly N chars.
    // If input is shorter, we process what exists (narrow behavior without inventing errors).
    let bytes = s.as_bytes();
    let limit = n.min(bytes.len());

    for i in 0..limit {
        let required = char_to_bit(bytes[i]) as usize;

        // Clear next
        next.fill(0);

        // Transition:
        // For each previous held-mask p and new held-mask q:
        // - q must include the required key for this day
        // - q must share at least one key with p (continuity of "held over the sequence")
        // Initialization (i==0): start from a fixed initial holder containing 'J' (mask 1),
        // which is the narrowest deterministic choice consistent with "keys held over the sequence".
        if i == 0 {
            let initial = 1usize; // 'J'
            for q in 1..=7usize {
                if (q & required) != 0 && (q & initial) != 0 {
                    next[q] = (next[q] + 1) % MOD;
                }
            }
        } else {
            for p in 1..=7usize {
                let pv = prev[p];
                if pv == 0 {
                    continue;
                }
                for q in 1..=7usize {
                    if (q & required) != 0 && (q & p) != 0 {
                        let mut v = next[q] + pv;
                        v %= MOD;
                        next[q] = v;
                    }
                }
            }
        }

        prev = next;
    }

    // Total combinations: sum of all valid end masks modulo MOD.
    let mut ans: i32 = 0;
    for m in 1..=7usize {
        ans += prev[m];
        ans %= MOD;
    }

    // Postcondition: non-negative and < MOD.
    if ans < 0 {
        ans = (ans % MOD + MOD) % MOD;
    }
    println!("{}", ans);
}
