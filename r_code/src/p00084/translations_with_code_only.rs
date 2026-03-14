use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    
    // Create a mutable string to hold the processed line
    let mut processed_line = line.trim_end().to_string();
    
    // Define the delimiters
    let delimiters = [' ', ',', '.'];
    
    // Split the line by the defined delimiters
    let mut sign1 = false;
    let mut output = String::new();
    
    for word in processed_line.split(|c| delimiters.contains(&c)) {
        let len = word.len();
        if len >= 3 && len <= 6 {
            if sign1 {
                output.push(' ');
            } else {
                sign1 = true;
            }
            output.push_str(word);
        }
    }
    
    // Print the output followed by a newline
    println!("{}", output);
}
