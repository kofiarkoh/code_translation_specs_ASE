fn main() {
    // Read the number of rows in the character grid
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    // Validate N
    if n == 0 || n > 52 {
        panic!("N must be a positive integer less than or equal to 52");
    }

    // Initialize the character grid
    let mut map: [[char; 53]; 53] = [['\0'; 53]; 53];

    // Read the character grid
    for i in 0..n {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let line: Vec<char> = input.trim().chars().collect();

        // Validate the length of each row
        if line.len() != 52 {
            panic!("Each row in map must contain exactly 52 characters");
        }

        // Fill the grid
        for j in 0..52 {
            map[i][j] = line[j];
        }
    }

    // Frequency array for characters
    let mut frequency = [0; 52]; // 26 uppercase + 26 lowercase

    // Calculate frequencies
    for i in 0..n {
        for j in 0..52 {
            let c = map[i][j];
            if c.is_ascii_uppercase() {
                frequency[(c as usize) - ('A' as usize)] += 1;
            } else if c.is_ascii_lowercase() {
                frequency[(c as usize) - ('a' as usize) + 26] += 1;
            }
        }
    }

    // Output the decoded message
    for i in 0..26 {
        if frequency[i] > 0 {
            print!("{}", (b'A' + i as u8) as char);
            for _ in 1..frequency[i] {
                print!("{}", (b'A' + i as u8) as char);
            }
        }
    }
    for i in 0..26 {
        if frequency[i + 26] > 0 {
            print!("{}", (b'a' + i as u8) as char);
            for _ in 1..frequency[i + 26] {
                print!("{}", (b'a' + i as u8) as char);
            }
        }
    }
    println!(); // Newline at the end of output
}
