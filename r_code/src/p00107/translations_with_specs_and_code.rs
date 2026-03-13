use std::io::{self, Read};

fn main() {
    // Read all input tokens (whitespace-separated)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut parts = input.split_whitespace();
    let mut out = String::new();

    // Loop processing blocks as per SPEC:
    // Repeatedly read t0, t1, t2; if any is zero -> terminate immediately.
    // Otherwise read n, then read n data integers and for each print "OK" or "NA".
    loop {
        // Read triple t0, t1, t2. If any missing -> terminate.
        let t0 = match parts.next() {
            Some(tok) => tok.parse::<i64>().unwrap(),
            None => break,
        };
        let t1 = match parts.next() {
            Some(tok) => tok.parse::<i64>().unwrap(),
            None => break,
        };
        let t2 = match parts.next() {
            Some(tok) => tok.parse::<i64>().unwrap(),
            None => break,
        };

        // Terminate immediately if any t is zero (no further reading for this block).
        if t0 == 0 || t1 == 0 || t2 == 0 {
            break;
        }

        // Read n. If missing -> terminate.
        let n = match parts.next() {
            Some(tok) => tok.parse::<i64>().unwrap(),
            None => break,
        };

        // Selection of two t-values follows the SPEC rules:
        // If a >= c -> select (c, b)
        // Else if b >= c -> select (a, c)
        // Else select (a, b)
        let (a, b, c) = (t0, t1, t2);
        let (sel1, sel2) = if a >= c {
            (c, b)
        } else if b >= c {
            (a, c)
        } else {
            (a, b)
        };

        // Compute S = sel1^2 + sel2^2 using a larger integer type to avoid accidental overflow.
        // The SPEC notes that the original C code did not guard against overflow; using i128
        // here is a pragmatic choice to preserve intended numerical comparisons for typical inputs.
        let sel1_i128 = sel1 as i128;
        let sel2_i128 = sel2 as i128;
        let s_sum = sel1_i128 * sel1_i128 + sel2_i128 * sel2_i128;

        // If n <= 0, the program should perform zero iterations for the data-reading loop.
        if n > 0 {
            // For each of the next n integers x: compute D = 4*x^2 and compare D > S.
            // If input runs out early, terminate processing.
            for _ in 0..n {
                let x = match parts.next() {
                    Some(tok) => tok.parse::<i64>().unwrap(),
                    None => {
                        // Input ended prematurely; per SPEC behaviour is undefined in C.
                        // Here we gracefully terminate.
                        print!("{}", out);
                        return;
                    }
                };
                let x_i128 = x as i128;
                let d = 4 * x_i128 * x_i128;
                if d > s_sum {
                    out.push_str("OK\n");
                } else {
                    out.push_str("NA\n");
                }
            }
        }
        // After processing the block, continue to next triple.
    }

    // Flush output
    print!("{}", out);
}
