use std::io;

fn main() {
    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Define the maximum size of the array
    const MAX_SIZE: usize = 1000; // Example maximum size, can be adjusted
    assert!(n > 0 && n <= MAX_SIZE, "n must be a positive integer less than or equal to MAX_SIZE");

    // Read the integers into a vector
    let mut array = Vec::with_capacity(n);
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for num in input.trim().split_whitespace() {
        let value: i32 = num.parse().expect("Invalid integer");
        array.push(value);
    }

    // Find the pivot element
    let pivot_index = partition(&mut array);

    // Output the array with the pivot highlighted
    for (i, &value) in array.iter().enumerate() {
        if i == pivot_index {
            print!("[{}]", value);
        } else {
            // Print a space before the value if it's not the first element
            if i > 0 {
                print!(" ");
            }
            print!("{}", value);
        }
    }
    println!(); // Newline at the end of output
}

// Function to partition the array and find the pivot index
fn partition(array: &mut Vec<i32>) -> usize {
    let n = array.len();
    // Using the last element as the pivot
    let pivot = array[n - 1];
    let mut i = 0;

    for j in 0..n - 1 {
        if array[j] <= pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    // Place the pivot in its correct position
    array.swap(i, n - 1);
    i // Return the index of the pivot
}
