use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..n {
        let mut l = ['\0'; 27]; // Initialize an array of 27 chars
        let mut p = 13; // Start at index 13
        
        // Read characters until we hit EOF or a break condition
        for c in lines.next().unwrap().chars() {
            if c == '-' || c == '<' {
                if c == '-' {
                    p += 1; // Move right
                } else {
                    p -= 1; // Move left
                }
            } else {
                // Break on any other character
                break;
            }
            // Read the next character into l[p]
            if let Some(next_char) = lines.next().and_then(|line| line.chars().next()) {
                l[p] = next_char;
            } else {
                break; // Break if there's no more input
            }
        }
        
        // Find the first non-null character
        let start_index = l.iter().position(|&x| x != '\0').unwrap_or(0);
        // Print the string starting from the first non-null character
        println!("{}", l[start_index..].iter().collect::<String>());
    }
}
