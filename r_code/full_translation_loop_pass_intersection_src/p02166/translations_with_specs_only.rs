use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // unspecified behavior; choose to exit quietly if no input
    };
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Sentinel value: 0 indicates "absent/terminated" (matches typical 1-based mapping with 0 as null).
    // The spec does not name the sentinel explicitly; we choose the narrowest reasonable interpretation:
    // a value that is not a valid 1-based index, i.e., 0.
    let mut a: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        if v <= 0 {
            a.push(0);
        } else {
            a.push(v as usize);
        }
    }

    if k == n as i64 {
        // Uniform cyclic shift check: exists constant offset d such that (a[i] - (i+1)) mod n is constant.
        // Preconditions guarantee all a[i] are valid 1-based indices and arithmetic is modulo n.
        if n == 0 {
            // Not expected by preconditions (positive size), but keep behavior defined.
            println!("Yes");
            return;
        }

        let first = a[0];
        // Compute d in [0, n-1] using 1-based positions.
        let d0 = (first + n - 1) % n;

        let mut ok = true;
        for (i, &val) in a.iter().enumerate() {
            let pos1 = i + 1; // 1-based
            let di = (val + n - (pos1 % n)) % n;
            if di != d0 {
                ok = false;
                break;
            }
        }

        println!("{}", if ok { "Yes" } else { "No" });
    } else {
        // Mapping/parity case: destructively mark visited positions as sentinel (0).
        // Count components discovered by starting from each position that is not already sentinel.
        let mut components: usize = 0;

        for i in 0..n {
            if a[i] == 0 {
                continue;
            }
            components += 1;

            // Follow mapping until sentinel encountered, marking visited positions as sentinel.
            let mut cur = i + 1; // current index in 1-based space
            loop {
                let idx0 = cur - 1; // convert to 0-based
                let next = a[idx0];
                a[idx0] = 0; // once sentinel, remains sentinel
                if next == 0 {
                    break;
                }
                cur = next;
            }
        }

        // Output "No" iff (n - components) is odd AND k is odd; else "Yes".
        let parity_n_minus_c = ((n - components) & 1) as i64;
        let parity_k = (k & 1).abs(); // k is an integer parameter; treat oddness by low bit
        let no = parity_n_minus_c == 1 && parity_k == 1;

        println!("{}", if no { "No" } else { "Yes" });
    }
}
