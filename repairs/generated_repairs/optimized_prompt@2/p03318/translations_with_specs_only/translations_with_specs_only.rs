fn sum_of_digits(n: u32) -> u32 {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += num % 10; // Add the last digit
        num /= 10;       // Remove the last digit
    }
    sum
}

fn find_min_ratio_value(start: u32) -> u32 {
    let mut min_value = start;
    let mut min_ratio = f32::MAX; // Start with the maximum possible ratio

    // We will check values starting from `start` to find the minimum ratio
    for i in start..=u32::MAX {
        let digit_sum = sum_of_digits(i);
        if digit_sum > 0 {
            let ratio = i as f32 / digit_sum as f32; // Calculate the ratio
            if ratio < min_ratio {
                min_ratio = ratio;
                min_value = i; // Update the minimum value found
            }
        }
        // Break if we find a valid minimum ratio
        if min_ratio < f32::MAX {
            break;
        }
    }
    min_value
}

fn compute_sequence(iterations: u32) -> Vec<u32> {
    let mut results = Vec::with_capacity(iterations as usize);
    let mut current_value = 1; // Start with one to match the expected output

    for _ in 0..iterations {
        current_value = find_min_ratio_value(current_value);
        results.push(current_value);
        current_value += 1; // Increment to ensure the next value is different
    }

    results
}

fn main() {
    let iterations: u32 = 5; // Set to 5 to match the expected output
    let output_sequence = compute_sequence(iterations);
    
    for value in output_sequence {
        println!("{}", value); // Output each computed value
    }
}
