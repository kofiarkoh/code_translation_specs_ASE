use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the number of height measurements
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    // Ensure n is greater than 0 as per the specification
    if n == 0 {
        return; // Early exit if n is not valid
    }

    let mut counts = [0; 6]; // Array to hold counts for each category

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let h: f64 = input.trim().parse().expect("Please enter a valid height");

        // Categorize the height into the appropriate range
        if h < 165.0 {
            counts[0] += 1; // Category 1
        } else if h < 170.0 {
            counts[1] += 1; // Category 2
        } else if h < 175.0 {
            counts[2] += 1; // Category 3
        } else if h < 180.0 {
            counts[3] += 1; // Category 4
        } else if h < 185.0 {
            counts[4] += 1; // Category 5
        } else {
            counts[5] += 1; // Category 6
        }
    }

    // Output the histogram
    for (i, &count) in counts.iter().enumerate() {
        print!("{}:", i + 1);
        for _ in 0..count {
            print!("*");
        }
        println!(); // Print newline after each category
    }
}
