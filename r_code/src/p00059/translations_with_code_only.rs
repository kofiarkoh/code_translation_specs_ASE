use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string
    let mut s = String::new();
    if io::stdin().read_to_string(&mut s).is_err() {
        // If reading fails, just exit (similar to C program which would likely terminate)
        return;
    }

    // Create an iterator over whitespace-separated tokens.
    // We'll emulate scanf's behavior by stopping processing if a token is missing
    // or cannot be parsed as a floating point number.
    let mut toks = s.split_whitespace();

    loop {
        // Attempt to read 8 f64 values in the same order as the C scanf:
        // x0 y0 x1 y1 x2 y2 x3 y3
        let mut x = [0f64; 4];
        let mut y = [0f64; 4];
        let mut ok = true;

        for i in 0..4 {
            match toks.next() {
                Some(tok) => match tok.parse::<f64>() {
                    Ok(v) => x[i] = v,
                    Err(_) => { ok = false; break; },
                },
                None => { ok = false; break; },
            }
            match toks.next() {
                Some(tok) => match tok.parse::<f64>() {
                    Ok(v) => y[i] = v,
                    Err(_) => { ok = false; break; },
                },
                None => { ok = false; break; },
            }
            if !ok {
                break;
            }
        }

        if !ok {
            // If we couldn't read a full group of 8 parsable doubles, stop.
            // This mirrors scanf stopping on EOF or matching failure.
            break;
        }

        // Translate the C logic exactly
        if x[0] <= x[2] && x[1] >= x[2] {
            if (y[0] <= y[2] && y[1] >= y[2]) || (y[0] <= y[3] && y[1] >= y[3]) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else if x[0] <= x[3] && x[1] >= x[3] {
            if (y[0] <= y[2] && y[1] >= y[2]) || (y[0] <= y[3] && y[1] >= y[3]) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
        }
    }
}
