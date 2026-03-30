use std::io;

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut nums = input.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        // Ensure we have exactly two integers
        if nums.len() != 2 {
            continue; // Skip invalid input
        }

        let n = nums[0];
        let x = nums[1];

        // Termination condition
        if n == 0 && x == 0 {
            break;
        }

        // If n is less than 3, output is always zero
        if n < 3 {
            println!("0");
            continue;
        }

        let mut count = 0;

        // Count unique combinations of three distinct integers
        for i in 1..=n {
            for j in (i + 1)..=n {
                for k in (j + 1)..=n {
                    if i + j + k == x {
                        count += 1;
                    }
                }
            }
        }

        // Output the count divided by 6
        println!("{}", count);
    }
}
