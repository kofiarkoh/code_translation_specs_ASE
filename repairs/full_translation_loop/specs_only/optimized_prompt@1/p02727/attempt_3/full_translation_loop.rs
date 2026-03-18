use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Changed to mutable to allow multiple reads
    let mut input = String::new();
    
    // Read the first line of input
    if let Err(_) = reader.read_line(&mut input) {
        eprintln!("Error reading input");
        std::process::exit(1);
    }

    // Parse the first five integers: X, Y, A, B, C
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    if numbers.len() < 5 {
        eprintln!("Not enough integers provided");
        std::process::exit(1);
    }

    let (x, y, a, b, c) = (numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]);

    // Read the first array
    let mut first_array = Vec::new();
    for _ in 0..a {
        let mut line = String::new();
        if let Err(_) = reader.read_line(&mut line) {
            eprintln!("Error reading first array");
            std::process::exit(1);
        }
        if let Ok(num) = line.trim().parse::<i32>() {
            first_array.push(num);
        } else {
            eprintln!("Invalid integer in first array");
            std::process::exit(1);
        }
    }

    // Read the second array
    let mut second_array = Vec::new();
    for _ in 0..b {
        let mut line = String::new();
        if let Err(_) = reader.read_line(&mut line) {
            eprintln!("Error reading second array");
            std::process::exit(1);
        }
        if let Ok(num) = line.trim().parse::<i32>() {
            second_array.push(num);
        } else {
            eprintln!("Invalid integer in second array");
            std::process::exit(1);
        }
    }

    // Read the third array
    let mut third_array = Vec::new();
    for _ in 0..c {
        let mut line = String::new();
        if let Err(_) = reader.read_line(&mut line) {
            eprintln!("Error reading third array");
            std::process::exit(1);
        }
        if let Ok(num) = line.trim().parse::<i32>() {
            third_array.push(num);
        } else {
            eprintln!("Invalid integer in third array");
            std::process::exit(1);
        }
    }

    // Sort the first and second arrays in descending order
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

    // Calculate the sum of the first X integers from the first array and the first Y integers from the second array
    let sum_first = first_array.iter().take(x as usize).sum::<i32>();
    let sum_second = second_array.iter().take(y as usize).sum::<i32>();

    // Output the result
    println!("{}", sum_first + sum_second);
}
``` 

### Changes Made:
1. **Input Handling**: Ensure that the input is read correctly and that the expected number of integers is processed.
2. **Error Messages**: The error messages are retained to help identify where the input might be failing.
3. **Array Processing**: The logic for processing the third array remains unchanged, as it is necessary for the intended behavior. 

Make sure to test with the provided input files to ensure that the program behaves as expected.
