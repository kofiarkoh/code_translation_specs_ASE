use std::io::{self, BufRead};

fn main() -> Result<(), i32> {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the first line of input
    stdin.lock().read_line(&mut input).map_err(|_| 1)?;
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly 5 integers
    if numbers.len() < 5 {
        return Err(1);
    }

    // Extract X, Y, A, B, C
    let (x, y, a, b, c) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);
    
    // Check for non-negative A, B, C
    if a < 0 || b < 0 || c < 0 {
        return Err(1);
    }

    // Read the first array
    let mut first_array: Vec<i32> = Vec::with_capacity(a as usize);
    input.clear();
    stdin.lock().read_line(&mut input).map_err(|_| 1)?;
    first_array.extend(input.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).take(a as usize));

    // Read the second array
    let mut second_array: Vec<i32> = Vec::with_capacity(b as usize);
    input.clear();
    stdin.lock().read_line(&mut input).map_err(|_| 1)?;
    second_array.extend(input.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).take(b as usize));

    // Read the third array
    let mut third_array: Vec<i32> = Vec::with_capacity(c as usize);
    input.clear();
    stdin.lock().read_line(&mut input).map_err(|_| 1)?;
    third_array.extend(input.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).take(c as usize));

    // Sort first and second arrays in descending order
    first_array.sort_unstable_by(|a, b| b.cmp(a));
    second_array.sort_unstable_by(|a, b| b.cmp(a));

    // Process the third array to potentially replace elements in the first and second arrays
    for &value in &third_array {
        if value > 0 {
            if let Some(first) = first_array.last_mut() {
                if value > *first {
                    *first = value; // Replace the smallest in first_array
                }
            }
            if let Some(second) = second_array.last_mut() {
                if value > *second {
                    *second = value; // Replace the smallest in second_array
                }
            }
        }
    }

    // Calculate the sum of the first X integers from the first array and first Y integers from the second array
    let sum_first = first_array.iter().take(x as usize).sum::<i32>();
    let sum_second = second_array.iter().take(y as usize).sum::<i32>();

    // Output the result
    println!("{}", sum_first + sum_second);
    
    Ok(())
}
