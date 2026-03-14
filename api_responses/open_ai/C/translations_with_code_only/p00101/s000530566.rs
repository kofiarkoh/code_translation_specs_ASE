use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input_lines = handle.lines();

    let mut n = String::new();
    // Read the first line to get the number of iterations
    input_lines.next().unwrap().unwrap().trim().parse::<i32>().map(|x| n = x.to_string()).unwrap();

    for _ in 0..n.parse::<i32>().unwrap() {
        // Read each line of input
        let mut in_line = input_lines.next().unwrap().unwrap();
        // Replace "Hoshino" with "Hoshina"
        let mut start = 0;
        while let Some(pos) = in_line[start..].find("Hoshino") {
            let index = start + pos;
            // Change the character at the end of "Hoshino" to 'a'
            let bytes = in_line.as_bytes_mut();
            bytes[index + 6] = b'a';
            start = index + 1; // Move past the current match
        }
        // Print the modified line
        print!("{}", in_line);
    }
}
