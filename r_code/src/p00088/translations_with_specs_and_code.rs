use std::io::{self, BufRead};

fn main() {
    let a = "\n ',-.?ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let b: [&str; 32] = [
        "101", "101", "000000", "000011", "10010001", "010001", "000001", "100101",
        "10011010", "0101", "0001", "110", "01001", "10011011", "010000", "0111",
        "10011000", "0110", "00100", "10011001", "10011110", "00101", "111",
        "10011111", "1000", "00110", "00111", "10011100", "10011101", "000010",
        "10010010", "10010011", "10010000"
    ];
    let c = "ABCDEFGHIJKLMNOPQRSTUVWXYZ .,-'?";
    let mut d = String::new();
    
    // Read input until newline
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    for k in input.chars() {
        // Find the index of the character in `a`
        if let Some(j) = a.chars().position(|x| x == k) {
            d.push_str(b[j]);
        }
    }
    
    // Pad with '0' to ensure length is a multiple of 5
    while d.len() % 5 != 0 {
        d.push('0');
    }
    
    // Convert binary string to characters
    let mut output = String::new();
    let mut i = 0;
    
    while i < d.len() {
        let mut k = 0;
        let mut j = 4;
        
        // Convert the next 5 bits to a character
        while j >= 0 {
            k |= (d.chars().nth(i).unwrap() == '1') as usize << j;
            i += 1;
            j -= 1;
        }
        
        // Append the corresponding character from `c`
        output.push(c.chars().nth(k).unwrap());
    }
    
    // Print the output followed by a newline
    println!("{}", output);
}
