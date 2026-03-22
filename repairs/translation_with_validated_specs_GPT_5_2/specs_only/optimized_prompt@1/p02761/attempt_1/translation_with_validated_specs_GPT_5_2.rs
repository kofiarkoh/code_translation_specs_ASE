use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Special-case behavior when M = 0.
    if m == 0 {
        // Specification defines outputs only for N in {1,2,3}.
        let out = match n {
            1 => 0,
            2 => 10,
            3 => 100,
            _ => {
                // Behavior outside intended range is not well-defined; choose narrow behavior:
                // do nothing further and print -1 to avoid inventing extra functionality.
                println!("-1");
                return;
            }
        };
        println!("{}", out);
        return;
    }

    // Digits are stored as Option<u8> for positions 1..=N (most significant to least).
    let mut digits: Vec<Option<u8>> = vec![None; n];

    // Track the first provided constraint for the special error rule.
    let first_s: usize = it.next().unwrap().parse().unwrap();
    let first_c: u8 = it.next().unwrap().parse().unwrap();

    // Special rule: if (and only if) the first provided constraint explicitly sets
    // the most significant digit to 0 while N > 1, output -1 regardless of other constraints.
    if n > 1 && first_s == 1 && first_c == 0 {
        println!("-1");
        return;
    }

    // Apply the first constraint, checking for contradictions.
    if first_s >= 1 && first_s <= n {
        let idx = first_s - 1;
        if let Some(prev) = digits[idx] {
            if prev != first_c {
                println!("-1");
                return;
            }
        } else {
            digits[idx] = Some(first_c);
        }
    } else {
        // Out-of-range behavior is not well-defined; choose narrow behavior: print -1.
        println!("-1");
        return;
    }

    // Apply remaining constraints.
    for _ in 1..m {
        let s: usize = it.next().unwrap().parse().unwrap();
        let c: u8 = it.next().unwrap().parse().unwrap();

        if s < 1 || s > n {
            // Not well-defined by spec; choose narrow behavior.
            println!("-1");
            return;
        }
        let idx = s - 1;
        if let Some(prev) = digits[idx] {
            if prev != c {
                println!("-1");
                return;
            }
        } else {
            digits[idx] = Some(c);
        }
    }

    // Fill unconstrained digits to make the number as small as possible,
    // ensuring a nonzero most significant digit when N > 1.
    for i in 0..n {
        if digits[i].is_none() {
            if i == 0 && n > 1 {
                digits[i] = Some(1);
            } else {
                digits[i] = Some(0);
            }
        }
    }

    // Enforce invariant: when producing a valid result for N > 1, MS digit is nonzero.
    if n > 1 && digits[0] == Some(0) {
        println!("-1");
        return;
    }

    // Construct the integer (N intended <= 3, so i32 is sufficient).
    let mut value: i32 = 0;
    for d in digits {
        value = value * 10 + d.unwrap() as i32;
    }

    println!("{}", value);
}
