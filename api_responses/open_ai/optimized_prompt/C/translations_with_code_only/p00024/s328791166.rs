use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(v) = line.trim().parse::<f32>() {
            // Calculate n based on the formula from the C code
            let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
            let n = n.floor() as i32 + 1; // Floor the value and add 1
            println!("{}", n);
        } else {
            // If parsing fails, we can choose to ignore or handle it as needed
            // In the original C code, invalid input would just skip the iteration
            continue;
        }
    }
}
