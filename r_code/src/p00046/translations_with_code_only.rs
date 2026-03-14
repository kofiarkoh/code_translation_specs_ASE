use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    let mut first = false;
    let mut hm: f32 = 0.0;
    let mut lm: f32 = 0.0;

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(m) = line.trim().parse::<f32>() {
            if !first {
                first = true;
                hm = m;
                lm = m;
            }
            if hm < m {
                hm = m;
            } else if lm > m {
                lm = m;
            }
        }
    }

    // Print the difference with one decimal place
    println!("{:.1}", hm - lm);
}
