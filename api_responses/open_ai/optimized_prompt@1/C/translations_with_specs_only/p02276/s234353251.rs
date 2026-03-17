fn main() {
    use std::io::{self, Write};

    // Define the maximum size of the array
    const MAX_SIZE: usize = 1000; // Example maximum size, can be adjusted as needed

    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Ensure n is within the valid range
    if n == 0 || n > MAX_SIZE {
        panic!("n must be a positive integer less than or equal to {}", MAX_SIZE);
    }

    // Read the integers into an array
    let mut array = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.trim().split_whitespace() {
        let value: i32 = num.parse().expect("Invalid integer");
        array.push(value);
    }

    // Ensure we have exactly n elements
    if array.len() != n {
        panic!("Expected {} integers, but got {}", n, array.len());
    }

    // Perform the partitioning to find the pivot
    let pivot_index = partition(&mut array);

    // Output the result with the pivot highlighted
    for (i, &value) in array.iter().enumerate() {
        if i == pivot_index {
            print!("[{}]", value); // Highlight the pivot
        } else {
            print!(" {}", value);
        }
    }
    println!(); // Newline at the end of output
}

// Function to partition the array and find the pivot index
fn partition(array: &mut Vec<i32>) -> usize {
    let len = array.len();
    let pivot_index = len / 2; // Choose the middle element as the pivot
    let pivot_value = array[pivot_index];

    // Rearrange the array around the pivot
    let mut left = 0;
    let mut right = len - 1;

    while left <= right {
        while left < len && array[left] <= pivot_value {
            left += 1;
        }
        while right > 0 && array[right] > pivot_value {
            right -= 1;
        }
        if left < right {
            array.swap(left, right);
        }
    }

    // Place the pivot in its correct position
    array.swap(pivot_index, right);
    right // Return the index of the pivot
}
