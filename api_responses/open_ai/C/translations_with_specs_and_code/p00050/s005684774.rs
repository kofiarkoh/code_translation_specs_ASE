use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap_or(Ok(String::new())).unwrap();

    // Process the input string
    let output = replace_fruits(&input);

    // Print the modified string
    println!("{}", output);
}

fn replace_fruits(input: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let n = input.len();

    while i < n {
        if i <= n - 5 && &input[i..i + 5] == "apple" {
            result.push('p');
            result.push('e');
            result.push('a');
            result.push('c');
            result.push('h');
            i += 5; // Move past the matched substring
        } else if i <= n - 5 && &input[i..i + 5] == "peach" {
            result.push('a');
            result.push('p');
            result.push('p');
            result.push('l');
            result.push('e');
            i += 5; // Move past the matched substring
        } else if i <= n - 5 && &input[i..i + 5] == "Apple" {
            result.push('P');
            result.push('e');
            result.push('a');
            result.push('c');
            result.push('h');
            i += 5; // Move past the matched substring
        } else if i <= n - 5 && &input[i..i + 5] == "Peach" {
            result.push('A');
            result.push('p');
            result.push('p');
            result.push('l');
            result.push('e');
            i += 5; // Move past the matched substring
        } else {
            result.push(input.chars().nth(i).unwrap()); // Push the current character
            i += 1; // Move to the next character
        }
    }

    result
}
