use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the integer N
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Input error");
        std::process::exit(1); // Input error
    }
    
    let n: usize = match input.trim().parse() {
        Ok(num) if num <= 127 => num,
        _ => {
            eprintln!("Input error");
            std::process::exit(1); // Input error
        }
    };

    // Read the string s
    input.clear();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Input error");
        std::process::exit(1); // Input error
    }
    
    let s = input.trim();
    if s.len() < n {
        eprintln!("Input error");
        std::process::exit(1); // Input error
    }

    // Initialize the counter
    let mut kitaeri = 0;

    // Evaluate the string
    for i in 0..n {
        if s.chars().nth(i) == Some('R') {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    // Output the result
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
