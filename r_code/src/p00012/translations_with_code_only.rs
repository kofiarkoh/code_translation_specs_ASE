use std::io::{self, Read};

fn area(a: f64, b: f64, c: f64) -> f64 {
    // Heron's formula exactly as in the C code.
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

fn main() {
    // Read entire stdin (C's scanf loop reads until EOF).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If we can't read stdin for some reason, just exit.
        return;
    }

    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Process tokens in groups of 8, ignoring any trailing incomplete group,
    // which mirrors the C program expecting 8 values per successful scanf.
    let mut i = 0usize;
    while i + 7 < tokens.len() {
        // Parse 8 doubles; if any parse fails, stop processing further input.
        let x1 = match tokens[i].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let y1 = match tokens[i + 1].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let x2 = match tokens[i + 2].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let y2 = match tokens[i + 3].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let x3 = match tokens[i + 4].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let y3 = match tokens[i + 5].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let xp = match tokens[i + 6].parse::<f64>() { Ok(v) => v, Err(_) => break };
        let yp = match tokens[i + 7].parse::<f64>() { Ok(v) => v, Err(_) => break };

        // Distances using hypot, mirroring C's hypot behavior.
        let a = (x1 - x2).hypot(y1 - y2);
        let b = (x2 - x3).hypot(y2 - y3);
        let c = (x1 - x3).hypot(y1 - y3);

        let ap = (x1 - xp).hypot(y1 - yp);
        let bp = (x2 - xp).hypot(y2 - yp);
        let cp = (xp - x3).hypot(yp - y3);

        let mut s = area(a, ap, bp);
        s += area(b, bp, cp);
        s += area(c, cp, ap);

        let S = area(a, b, c);

        // Compare with the same epsilon as the C code.
        if (s - S).abs() < 0.00000001 {
            println!("YES");
        } else {
            println!("NO");
        }

        i += 8;
    }
}
