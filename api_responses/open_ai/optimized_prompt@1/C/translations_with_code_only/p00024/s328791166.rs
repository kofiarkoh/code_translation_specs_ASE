use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap is safe here as we control input
        if let Ok(v) = line.trim().parse::<f32>() {
            // Calculate n based on the formula from the C code
            let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
            let n = n.floor() as i32 + 1; // Floor to mimic C's integer division behavior
            println!("{}", n);
        }
    }
}
