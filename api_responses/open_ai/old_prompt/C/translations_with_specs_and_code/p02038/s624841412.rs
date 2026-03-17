use std::io::{self, BufRead};

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' {
        'F'
    } else {
        'T'
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of boolean values
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().unwrap_or(0),
        _ => return println!(""), // Return without output on invalid input
    };

    // Validate N
    if n == 0 {
        return println!(""); // Handle case where N is 0
    }

    let mut p: Vec<char> = Vec::with_capacity(n);

    // Read boolean values
    for _ in 0..n {
        let line = match lines.next() {
            Some(Ok(line)) => line,
            _ => return println!(""), // Return without output on invalid input
        };
        if let Some(&first_char) = line.trim().chars().next() {
            if first_char == 'T' || first_char == 'F' {
                p.push(first_char);
            } else {
                return println!(""); // Handle invalid character
            }
        }
    }

    // Compute the result using the souti function
    let mut notomamiko = souti(p[0], p[1]);
    for &value in &p[2..] {
        notomamiko = souti(notomamiko, value);
    }

    // Output the result
    println!("{}", notomamiko);
}
