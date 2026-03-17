// Dynamic array management and querying system for integers
struct DynamicArray {
    data: Vec<i32>,
    size: usize,
}

impl DynamicArray {
    // Create a new dynamic array of a given size (must be a power of two)
    fn new(size: usize) -> Self {
        assert!(size.is_power_of_two(), "Size must be a power of two");
        Self {
            data: vec![0; size],
            size,
        }
    }

    // Update the dynamic array at a specific index
    fn update(&mut self, index: usize, value: i32) {
        if index < self.size {
            self.data[index] = value;
        }
    }

    // Query the maximum value in the dynamic array
    fn max(&self) -> i32 {
        *self.data.iter().max().unwrap_or(&0) // Return 0 if the array is empty
    }
}

// Main function to process input and produce output
fn main() {
    let n: usize = 5; // Example input, should be read from actual input in practice
    let inputs = vec![1, 2, 3, 4, 5]; // Example input sequence

    // Preconditions check
    assert!(n > 0, "n must be a positive integer");
    assert!(inputs.len() == n, "Input sequence must match n");

    // Create a dynamic array of size that is a power of two
    let mut dynamic_array = DynamicArray::new(8); // Example size, should be determined based on n

    // Update the dynamic array with input values
    for (i, &value) in inputs.iter().enumerate() {
        dynamic_array.update(i, value);
    }

    // Calculate the sum of the first n natural numbers
    let sum_natural_numbers: i64 = (1..=n as i64).sum();

    // Get the maximum value from the dynamic array
    let max_value = dynamic_array.max() as i64;

    // Calculate the result as specified
    let result = sum_natural_numbers - max_value;

    // Output the result
    println!("{}", result);
}
