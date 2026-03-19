use std::io::{self, Read};

fn main() {
    // Fixed maximum capacity: 1<<18 = 262,144
    const CAP: usize = 1 << 18;

    // Read all input as integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // C: scanf("%d%*d",&n) reads n and ignores one extra int.
    // If missing, behavior is not inferable; we default to POSSIBLE (no constraints).
    let n0: i32 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => {
            print!("POSSIBLE\n");
            return;
        }
    };
    // Ignore one integer if present.
    let _ignored = it.next();

    // C uses `n` as a flag: nonzero => contradiction detected => prints "IMPOSSIBLE".
    // Start with "no contradiction detected".
    let mut contradiction = false;

    // m is a global zero-initialized array of ints; p points into it.
    // We'll model it as a Vec<i32> initialized to 0.
    let mut m = vec![0i32; CAP];

    // Process constraints as pairs (a,b) until EOF.
    loop {
        let a_opt = it.next();
        let b_opt = it.next();
        if a_opt.is_none() || b_opt.is_none() {
            break;
        }
        let a: i32 = a_opt.unwrap().parse().unwrap();
        let b: i32 = b_opt.unwrap().parse().unwrap();

        // C pointer selection:
        // p = m + (a-1 ? (b-n ? 0 : a) : b);
        //
        // Here `n` in the C expression is the same variable later set to 0 on contradiction.
        // That means after contradiction, `b-n` changes, but the invariant says the final
        // result remains IMPOSSIBLE; we preserve that by continuing to read but keeping
        // `contradiction` latched.
        //
        // We must preserve the exact index computation semantics:
        // - (a-1) is "true" iff a != 1
        // - (b-n) is "true" iff b != n_flag (where n_flag is 1 when no contradiction, 0 after)
        let n_flag: i32 = if contradiction { 0 } else { 1 };

        let idx_i32: i32 = if a - 1 != 0 {
            if b - n_flag != 0 {
                0
            } else {
                a
            }
        } else {
            b
        };

        // Spec precondition says indices are valid; C has undefined behavior otherwise.
        // We clamp to avoid Rust panics while keeping behavior "not inferable" for invalid input.
        // (This is a best-effort to avoid crashing; valid inputs follow the spec.)
        let mut idx = idx_i32;
        if idx < 0 {
            idx = 0;
        }
        let mut idx_usize = idx as usize;
        if idx_usize >= CAP {
            idx_usize = CAP - 1;
        }

        // C loop body:
        // p-m && *p ? n=0 : ++*p
        // i.e., if idx != 0 and m[idx] != 0 => contradiction; else increment m[idx].
        if idx_usize != 0 && m[idx_usize] != 0 {
            contradiction = true;
        } else {
            m[idx_usize] += 1;
        }
    }

    if contradiction {
        print!("IMPOSSIBLE\n");
    } else {
        print!("POSSIBLE\n");
    }
}
