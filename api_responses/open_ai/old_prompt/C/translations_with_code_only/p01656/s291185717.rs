use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut names: [[u8; 32]; 50] = [[0; 32]; 50];
    let mut answer = names[0].clone();
    
    // Read N and Q
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: i32 = iter.next().unwrap().parse().unwrap();

    // Read names and years
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let year: i32 = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap();

        // Copy the name into the names array
        let bytes = name.as_bytes();
        names[i].[..bytes.len()].copy_from_slice(bytes);
        
        // Check the year condition
        if year <= q {
            answer = names[i].clone();
        }
    }

    // Print the answer, converting from bytes to a string
    let answer_str = String::from_utf8_lossy(&answer);
    println!("{}", answer_str.trim_end_matches(char::from(0))); // Trim null bytes
}
