use std::io::{self, Write};

fn main() {
    let mut str = String::new();
    let mut strsub = String::with_capacity(21); // Allocate space for 21 characters

    // Read input from stdin
    io::stdin().read_line(&mut str).expect("Failed to read line");
    
    // Trim the input to remove any trailing newline characters
    let str = str.trim();

    // Ensure we only process up to 20 characters (to match C's behavior)
    let x = str.len().min(20);
    
    // Reverse the string manually to mimic the C code
    for i in 0..x {
        strsub.push(str.chars().nth(x - 1 - i).unwrap());
    }

    // Print the reversed string
    print!("{}", strsub);
    io::stdout().flush().expect("Failed to flush stdout");
}
