use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let mut s: Vec<String> = Vec::new();
    let mut count = 0;
    let mut max_length = 0;
    let mut ans1 = 100; // Initialized to a large value
    let mut ans2 = 100; // Initialized to a large value

    // Read characters until a newline is encountered
    let mut current_word = String::new();
    while let Some(Ok(ch)) = lines.next() {
        for c in ch.chars() {
            if c == '\n' {
                break;
            }
            if c == ' ' {
                if !current_word.is_empty() {
                    s.push(current_word.clone());
                    if max_length < current_word.len() {
                        max_length = current_word.len();
                        ans2 = count;
                    }
                    count += 1;
                    current_word.clear();
                }
            } else {
                current_word.push(c);
            }
        }
    }
    // Push the last word if it exists
    if !current_word.is_empty() {
        s.push(current_word.clone());
        if max_length < current_word.len() {
            max_length = current_word.len();
            ans2 = count;
        }
        count += 1;
    }

    // Find the most frequent word
    let mut max_count = 0;
    for i in 0..count {
        let mut c = 0;
        for j in (i + 1)..count {
            if s[i] == s[j] {
                c += 1;
            }
            if max_count < c {
                max_count = c;
                ans1 = i;
            }
        }
    }

    // Print the results
    println!("{} {}", s[ans1], s[ans2]);
}
