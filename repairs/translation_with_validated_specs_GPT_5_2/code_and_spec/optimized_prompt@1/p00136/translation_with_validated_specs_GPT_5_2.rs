use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens (matches scanf-style behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();

    let mut ca: usize = 0;
    let mut cb: usize = 0;
    let mut cc: usize = 0;
    let mut cd: usize = 0;
    let mut ce: usize = 0;
    let mut cf: usize = 0;

    // Spec assumes n is non-negative; if negative, behavior is unspecified (we follow that).
    for _ in 0..(n as usize) {
        let h: f64 = it.next().unwrap().parse().unwrap();
        if h < 165.0 {
            ca += 1;
        } else if h >= 165.0 && h < 170.0 {
            cb += 1;
        } else if h >= 170.0 && h < 175.0 {
            cc += 1;
        } else if h >= 175.0 && h < 180.0 {
            cd += 1;
        } else if h >= 180.0 && h < 185.0 {
            ce += 1;
        } else {
            // h >= 185.0
            cf += 1;
        }
    }

    // Exact output format: six lines, "k:" followed immediately by '*' repeated count times, then '\n'.
    let mut out = String::new();
    out.push_str("1:");
    out.push_str(&"*".repeat(ca));
    out.push('\n');

    out.push_str("2:");
    out.push_str(&"*".repeat(cb));
    out.push('\n');

    out.push_str("3:");
    out.push_str(&"*".repeat(cc));
    out.push('\n');

    out.push_str("4:");
    out.push_str(&"*".repeat(cd));
    out.push('\n');

    out.push_str("5:");
    out.push_str(&"*".repeat(ce));
    out.push('\n');

    out.push_str("6:");
    out.push_str(&"*".repeat(cf));
    out.push('\n');

    print!("{out}");
}
