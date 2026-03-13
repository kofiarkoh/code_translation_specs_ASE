use std::io::{self, Read};

fn main() {
    // Read entire stdin; the C program used scanf without robust error checking.
    // We follow the SPEC: if input is missing or malformed, we simply do nothing / exit.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut parts = input.split_whitespace();

    // Read n (number of test cases). If missing or not an integer, exit quietly.
    let n: i32 = match parts.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return,
    };

    if n <= 0 {
        // Per spec, no test cases => no output.
        return;
    }

    let mut out = String::new();

    // For each test case, read exactly 10 integers. If any are missing or fail to parse,
    // we stop processing further test cases (matching the spirit of the original C program
    // which didn't check scanf return values).
    for _ in 0..(n as usize) {
        let mut a = [0i32; 10];
        let mut ok = true;
        for j in 0..10 {
            match parts.next().and_then(|s| s.parse::<i32>().ok()) {
                Some(val) => a[j] = val,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }

        // Greedy two-track assignment per SPEC:
        let mut track1 = a[0];
        let mut track2 = 0i32;
        let mut rejected = false;

        for j in 1..10 {
            let x = a[j];
            if track1 < x {
                track1 = x;
            } else if track2 < x {
                track2 = x;
            } else {
                out.push_str("NO\n");
                rejected = true;
                break;
            }
        }

        if !rejected {
            out.push_str("YES\n");
        }
    }

    // Preserve exact stdout formatting: lines "YES" or "NO" each followed by newline.
    print!("{}", out);
}
