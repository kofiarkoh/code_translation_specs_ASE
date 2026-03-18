use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf-style whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();

    let mut ca = 0usize;
    let mut cb = 0usize;
    let mut cc = 0usize;
    let mut cd = 0usize;
    let mut ce = 0usize;
    let mut cf = 0usize;

    for _ in 0..n {
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

    // Preserve exact output formatting: label, asterisks, newline.
    println!("1:{}", "*".repeat(ca));
    println!("2:{}", "*".repeat(cb));
    println!("3:{}", "*".repeat(cc));
    println!("4:{}", "*".repeat(cd));
    println!("5:{}", "*".repeat(ce));
    println!("6:{}", "*".repeat(cf));
}
