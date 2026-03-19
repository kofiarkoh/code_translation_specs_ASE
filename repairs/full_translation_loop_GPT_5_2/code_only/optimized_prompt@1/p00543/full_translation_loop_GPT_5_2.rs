use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i32 like scanf("%d")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // no input -> do nothing (closest to C's behavior on missing input)
    };
    let m: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C uses a[1000] and indexes from 1..=n. Keep that layout.
    // If n > 999, C would write out of bounds (UB). We cannot/should not emulate UB;
    // we allocate enough to cover 1..=n safely.
    let mut a: Vec<i32> = vec![0; (n as usize) + 1];

    for i in 1..=n {
        if let Some(v) = it.next() {
            a[i as usize] = v.parse().unwrap();
        } else {
            // Missing numbers: C would leave uninitialized/old stack values (UB).
            // We keep default 0 for determinism and safety.
            a[i as usize] = 0;
        }
    }

    let mut k: i32 = 1;
    loop {
        // C: for(i=1,j=2; i<n, j<=n; i++,j++)
        // The comma operator makes the condition effectively (j<=n).
        // So the loop runs while j<=n, regardless of i<n.
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        while j <= n {
            // C's % on negative numbers truncates toward zero; Rust matches that for i32.
            // k is >= 1 here, so no division-by-zero.
            if a[i as usize] % k > a[j as usize] % k {
                let temp = a[i as usize];
                a[i as usize] = a[j as usize];
                a[j as usize] = temp;
            }
            i += 1;
            j += 1;
        }

        k += 1;
        if k > m {
            break;
        }
    }

    let mut out = String::new();
    for i in 1..=n {
        out.push_str(&format!("{}\n", a[i as usize]));
    }
    print!("{out}");
}
