struct DynamicArray {
    data: Vec<i32>,
    size: usize,
}

impl DynamicArray {
    fn new(size: usize) -> Self {
        // Initialize the dynamic array with a given size (must be a power of two)
        DynamicArray {
            data: vec![0; size],
            size,
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        // Update the value at the specified index
        if index < self.size {
            self.data[index] = value;
        }
        // No behavior defined for out-of-bounds index
    }

    fn max_in_range(&self, start: usize, end: usize) -> i32 {
        // Find the maximum value in the specified range
        // Behavior is undefined if the range exceeds the bounds of the array
        let mut max_value = i32::MIN;
        for i in start..end {
            if i < self.size {
                max_value = max_value.max(self.data[i]);
            }
        }
        max_value
    }
}

fn calculate_result(n: usize, positions: &[i32]) -> i64 {
    // Preconditions: n must be positive and positions must be valid
    let size = 1 << (n.next_power_of_two().trailing_zeros()); // Ensure size is a power of two
    let mut dynamic_array = DynamicArray::new(size);

    // Update the dynamic array with the input positions
    for (i, &pos) in positions.iter().enumerate() {
        dynamic_array.update(i, pos);
    }

    // Calculate the sum of the first n natural numbers
    let sum_n = (n * (n + 1)) / 2;

    // Get the maximum value from the dynamic array
    let max_value = dynamic_array.max_in_range(0, n);

    // Return the difference as a long integer
    sum_n as i64 - max_value as i64
}

fn main() {
    // Example usage
    let n = 5; // Example input
    let positions = vec![1, 2, 3, 4, 5]; // Example positions
    let result = calculate_result(n, &positions);
    println!("{}", result);
}
