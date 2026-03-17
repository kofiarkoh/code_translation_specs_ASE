use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut input = String::new();
    
    // Read the number of height measurements
    io::stdin().read_line(&mut input)?;
    let n: usize = input.trim().parse().expect("Invalid input for n");

    // Ensure n is greater than 0
    if n == 0 {
        return Ok(()); // No output if n is 0
    }

    let mut counts = [0; 6]; // Array to hold counts for each category

    // Read height measurements and categorize them
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input)?;
        let h: f64 = input.trim().parse().expect("Invalid input for height");

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
        println!(); // Newline after each category
    }

    Ok(())
}
