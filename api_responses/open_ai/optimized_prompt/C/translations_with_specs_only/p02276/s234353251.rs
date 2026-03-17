fn main() {
    // Read the number of elements in the array
    let n: usize = read_input(); // Assume read_input() reads an integer from input

    // Define the maximum size of the array
    const MAX_SIZE: usize = 100; // Example maximum size, can be adjusted as needed

    // Check precondition for n
    if n == 0 || n > MAX_SIZE {
        // Undefined behavior as per specification; no error handling implemented
        return;
    }

    // Read the array elements
    let mut array: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        let element: i32 = read_input(); // Assume read_input() reads an integer from input
        array.push(element);
    }

    // Perform partitioning; using the last element as the pivot
    let pivot_index = partition(&mut array);

    // Output the array with the pivot element highlighted
    for (i, &value) in array.iter().enumerate() {
        if i == pivot_index {
            print!("[{}]", value); // Highlight the pivot element
        } else {
            print!("{} ", value);
        }
    }
    println!(); // New line after output
}

// Function to partition the array and return the index of the pivot
fn partition(array: &mut Vec<i32>) -> usize {
    let pivot = *array.last().unwrap(); // Last element as pivot
    let mut i = 0;

    for j in 0..array.len() - 1 { // Exclude the pivot itself
        if array[j] < pivot {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, array.len() - 1); // Place pivot in its final position
    i // Return the index of the pivot
}

// Placeholder for input reading function
fn read_input() -> i32 {
    // This function should implement the logic to read an integer from input
    // For the sake of this example, we will return a dummy value
    0 // Replace with actual input reading logic
}
