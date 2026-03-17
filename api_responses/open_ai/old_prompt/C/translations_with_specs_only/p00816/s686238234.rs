fn func(s: &str, n: usize, m: usize) -> i32 {
    // Ensure the input string is valid and meets the specified constraints
    if n < 1 || n > 9 || s.len() != n {
        println!("error");
        return 0; // Return 0 as a non-negative integer
    }

    // Convert the string of digits into a vector of integers
    let digits: Vec<i32> = s.chars()
        .map(|c| c.to_digit(10).unwrap() as i32) // Convert char to digit
        .collect();

    let mut max_sum = 0;
    let mut best_partition = String::new();
    let mut found_valid_partition = false;
    let mut rejected = false;

    // Iterate through all possible partitions using the bitmask
    for mask in 0..(1 << (n - 1)) {
        let mut current_sum = 0;
        let mut current_partition = String::new();
        let mut last_index = 0;

        for i in 0..n {
            // Check if the current bit in the mask is set
            if (mask & (1 << i)) != 0 {
                // Form the number from last_index to i
                let number: i32 = digits[last_index..=i].iter().sum();
                current_sum += number;
                current_partition.push_str(&number.to_string());
                current_partition.push(' '); // Add space for separation
                last_index = i + 1; // Update last_index for the next number
            }
        }

        // Handle the last number after the loop
        if last_index < n {
            let number: i32 = digits[last_index..].iter().sum();
            current_sum += number;
            current_partition.push_str(&number.to_string());
        }

        // Check if the current sum is valid and if it exceeds the target
        if current_sum <= m {
            if current_sum > max_sum {
                max_sum = current_sum;
                best_partition = current_partition.trim_end().to_string(); // Remove trailing space
                found_valid_partition = true;
                rejected = false; // Reset rejected flag
            } else if current_sum == max_sum {
                rejected = true; // Found another partition with the same max sum
            }
        }
    }

    // Output the results based on the findings
    if found_valid_partition {
        if rejected {
            println!("rejected");
        } else {
            println!("{} {}", max_sum, best_partition);
        }
    } else {
        println!("error");
    }

    max_sum // Return the maximum sum found
}
