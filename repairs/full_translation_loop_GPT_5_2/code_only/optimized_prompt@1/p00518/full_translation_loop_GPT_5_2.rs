use std::io::{self, Read};

const MOD: i32 = 10007;

fn main() {
    // Read entire stdin, then parse like scanf("%d") and scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let s = it.next().unwrap_or("");

    let bytes = s.as_bytes();

    let mut key_prev: i32 = 1; // initially J has the key
    let mut _key_prev_prev: i32 = 0; // present in C but not used in logic

    let mut schedule = [0i32; 8];
    let mut schedule_next = [0i32; 8];

    schedule[1] = 1;

    for i in 0..n {
        // In the C code, missing default in switch would be UB if unexpected char.
        // We map only the expected cases; otherwise use 0 to keep behavior defined.
        let key: i32 = match bytes.get(i).copied() {
            Some(b'J') => 1,
            Some(b'O') => 2,
            Some(b'I') => 4,
            _ => 0,
        };

        schedule_next = [0i32; 8];

        if i == 0 {
            for j in 0..=7 {
                if (j as i32 & (key | key_prev)) == (key | key_prev) {
                    schedule_next[j] = 1;
                }
            }
            _key_prev_prev = key_prev;
            key_prev = key;
            schedule = schedule_next;
            continue;
        }

        for j in 0..=7 {
            // Note: C code uses != 0 (not the commented == check).
            if ((j as i32) & (key | key_prev)) != 0 {
                for k in 0..=7 {
                    if (((k as i32) & (j as i32)) != 0) && (((k as i32) & key) != 0) {
                        // Keep values reduced mod MOD to avoid overflow, matching C intent.
                        let mut v = schedule_next[k] + schedule[j];
                        v %= MOD;
                        schedule_next[k] = v;
                    }
                }
            }
        }

        _key_prev_prev = key_prev;
        key_prev = key;
        schedule = schedule_next;
    }

    let mut conv: i32 = 0;
    for i in 0..=7 {
        conv += schedule[i];
        conv %= MOD;
    }

    print!("{conv}\n");
}
