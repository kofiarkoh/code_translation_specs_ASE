use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the first input which is the number of subsequent values
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            eprintln!("The first input must be a positive integer.");
            return;
        }
    };

    // Initialize counts for each category
    let mut cnt = [0; 7]; // cnt[0] is unused, we use cnt[1] to cnt[6]

    // Read the subsequent floating-point values
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let sin: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid floating-point number.");
                return;
            }
        };

        // Categorize the input value
        if sin < 165.0 {
            cnt[1] += 1;
        } else if sin < 170.0 {
            cnt[2] += 1;
        } else if sin < 175.0 {
            cnt[3] += 1;
        } else if sin < 180.0 {
            cnt[4] += 1;
        } else if sin < 185.0 {
            cnt[5] += 1;
        } else {
            cnt[6] += 1;
        }
    }

    // Output the histogram
    for i in 1..=6 {
        print!("{}:", i);
        for _ in 0..cnt[i] {
            print!("*");
        }
        println!();
    }
}
