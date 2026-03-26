use std::io;

fn main() {
    let mut input = String::new();
    let mut names: [[u8; 32]; 50] = [[0; 32]; 50];
    let mut answer_index = 0; // Track the index of the answer

    // Initialize the first name
    names[0][..15].copy_from_slice(b"kogakubu10gokan\0");

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
        let name = parts.next().unwrap();

        // Copy the name into the names array
        let bytes = name.as_bytes();
        names[i][..bytes.len()].copy_from_slice(bytes);

        // Update answer index if the year is less than or equal to Q
        if year <= q {
            answer_index = i;
        }
    }

    // Print the answer as a string
    let answer_str = std::str::from_utf8(&names[answer_index]).unwrap();
    println!("{}", answer_str.trim_end_matches(char::from(0))); // Trim null bytes
}
