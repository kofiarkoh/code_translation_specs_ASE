use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let mut names: [[u8; 32]; 50] = [[0; 32]; 50];
    let mut answer = "kogakubu10gokan".as_bytes();

    // Read N and Q
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: i32 = iter.next().unwrap().parse().unwrap();

    // Read names and find the answer
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let year: i32 = parts.next().unwrap().parse().unwrap();
        let name = parts.next().unwrap().as_bytes();

        // If the year is less than or equal to Q, update the answer
        if year <= q {
            answer = name;
        }
    }

    // Print the answer as a string
    // We need to convert the byte slice back to a string
    let answer_str = std::str::from_utf8(answer).unwrap();
    println!("{}", answer_str.trim_end_matches(char::from(0))); // Trim null bytes if any
}
