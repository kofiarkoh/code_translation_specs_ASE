use std::io::{self, Read};

const LENGTH: usize = 1001;

fn main() {
    let mut st = [0u8; LENGTH]; // Using u8 to mimic char behavior in C
    let mut i = 0;

    // Read input until EOF
    let stdin = io::stdin();
    let mut buffer = Vec::new();
    stdin.lock().read_to_end(&mut buffer).unwrap();

    // Copy input into st, ensuring we don't exceed LENGTH
    for &ch in &buffer {
        if i < LENGTH - 1 { // Leave space for null terminator
            st[i] = ch;
            i += 1;
        } else {
            break; // Prevent overflow
        }
    }
    st[i] = 0; // Null terminator

    let k = i; // Length of the string

    let mut j = 1;
    let mut kosuu = 0;
    let mut i = 0;

    while i < k {
        if st[i] == b'@' {
            kosuu = (st[i + 1] - b'0') as usize; // Convert ASCII to integer
            for _ in 0..kosuu {
                print!("{}", st[i + 2] as char); // Print the character
            }
            i += 3; // Move past '@', count, and character
        } else if st[i] == b'\n' {
            println!(); // Print newline
            i += 1;
        } else if st[i] == 0 {
            break; // End of string
        } else {
            print!("{}", st[i] as char); // Print the character
            i += 1;
        }
    }
    println!(); // Final newline
}
