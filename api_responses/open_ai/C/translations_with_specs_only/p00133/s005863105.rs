fn main() {
    // Read the original 8x8 character pattern
    let mut pattern: [[char; 8]; 8] = [[' '; 8]; 8];
    
    for i in 0..8 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        // Trim the input to ensure it has exactly 8 characters
        let trimmed = input.trim();
        assert_eq!(trimmed.len(), 8, "Each row must contain exactly 8 characters");
        pattern[i] = trimmed.chars().collect::<Vec<char>>()[..8].try_into().unwrap();
    }

    // Function to rotate the pattern 90 degrees clockwise
    fn rotate_90(pattern: &[[char; 8]; 8]) -> [[char; 8]; 8] {
        let mut rotated = [[' '; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                rotated[j][7 - i] = pattern[i][j];
            }
        }
        rotated
    }

    // Perform rotations and display results
    let mut current_pattern = pattern;

    for angle in [90, 180, 270].iter() {
        current_pattern = rotate_90(&current_pattern);
        println!("{} degrees:", angle);
        for row in &current_pattern {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
