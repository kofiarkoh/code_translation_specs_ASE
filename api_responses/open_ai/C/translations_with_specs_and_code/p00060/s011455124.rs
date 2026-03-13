use std::io::{self, Read};

fn main() {
    // Read entire stdin. If reading fails, exit normally (return 0).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    // Iterate whitespace-separated tokens. Per SPEC, stop processing when
    // we cannot obtain/parse three integers for a record.
    let mut toks = input.split_whitespace();

    loop {
        // Attempt to read and parse three integers. If any step fails (EOF
        // or parse error), stop processing and exit normally.
        let s1 = match toks.next() {
            Some(s) => s,
            None => break,
        };
        let c1 = match s1.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        let s2 = match toks.next() {
            Some(s) => s,
            None => break,
        };
        let c2 = match s2.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        let s3 = match toks.next() {
            Some(s) => s,
            None => break,
        };
        let c3 = match s3.parse::<i64>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Count how many integers i in 1..=10 satisfy:
        // i != c1 && i != c2 && i != c3 && c1 + c2 + i <= 20
        // Use i64 to avoid overflow/panic for very large input integers.
        let mut cnt = 0;
        for i in 1i64..=10 {
            if c1 != i && c2 != i && c3 != i {
                if c1 + c2 + i <= 20 {
                    cnt += 1;
                }
            }
        }

        // Print exactly "YES\n" or "NO\n" per SPEC.
        if cnt >= 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

// Notes:
// - The original C code declared an unused `card` array; it has no effect on
//   observable behavior and is omitted here.
// - Parsing stops on any parse error per SPEC (no error messages are printed).
// - We use i64 for input integers to match the SPEC's allowance of very large
//   or negative inputs and to avoid overflow panics in Rust debug builds.
