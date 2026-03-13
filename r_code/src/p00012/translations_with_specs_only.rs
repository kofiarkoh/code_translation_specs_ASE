use std::io::{self, Read};

fn tri_area(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> f64 {
    // Area = |cross((x2-x1,y2-y1),(x3-x1,y3-y1))| / 2
    (((x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)).abs()) * 0.5
}

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    // Behavior: stop processing when fewer than 8 tokens remain or when a token fails to parse as f64.
    let mut input = String::new();
    // If reading fails, propagate the error by exiting (this is unlikely for stdin in normal use).
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut iter = input.split_whitespace();

    const TOL: f64 = 1e-8;

    loop {
        // Attempt to read exactly 8 f64 values for a test case.
        let mut vals = [0f64; 8];
        for i in 0..8 {
            match iter.next() {
                Some(tok) => match tok.parse::<f64>() {
                    Ok(v) => vals[i] = v,
                    // On non-numeric token, stop processing further test cases and exit normally.
                    Err(_) => return,
                },
                // EOF reached before 8 tokens: exit normally.
                None => return,
            }
        }

        let x1 = vals[0];
        let y1 = vals[1];
        let x2 = vals[2];
        let y2 = vals[3];
        let x3 = vals[4];
        let y3 = vals[5];
        let xp = vals[6];
        let yp = vals[7];

        let s_main = tri_area(x1, y1, x2, y2, x3, y3);
        let s1 = tri_area(xp, yp, x1, y1, x2, y2);
        let s2 = tri_area(xp, yp, x2, y2, x3, y3);
        let s3 = tri_area(xp, yp, x3, y3, x1, y1);
        let s_sum = s1 + s2 + s3;

        if (s_sum - s_main).abs() < TOL {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
