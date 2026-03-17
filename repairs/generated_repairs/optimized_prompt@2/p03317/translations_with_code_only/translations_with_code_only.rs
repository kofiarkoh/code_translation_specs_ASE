use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i64>());
    
    let n = match nums.next() {
        Some(Ok(value)) => value,
        _ => {
            eprintln!("Failed to parse n");
            return;
        }
    };
    
    let k = match nums.next() {
        Some(Ok(value)) => value,
        _ => {
            eprintln!("Failed to parse k");
            return;
        }
    };

    // Read array a
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        match line.parse::<i64>() {
            Ok(value) => a.push(value),
            Err(_) => {
                eprintln!("Failed to parse array element");
                return;
            }
        }
    }

    // Calculate t
    let mut t = (n - k) / (k - 1) + 1;
    if k > 1 && k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    // Print result
    println!("{}", t);
}
