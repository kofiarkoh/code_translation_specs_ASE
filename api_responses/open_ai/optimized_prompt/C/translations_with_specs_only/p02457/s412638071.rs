struct DynamicArray {
    elements: Vec<i32>, // Vector to hold the elements
    count: usize,       // Current count of elements
}

impl DynamicArray {
    // Initialize the dynamic array with a specified capacity
    fn new(capacity: usize) -> Self {
        DynamicArray {
            elements: Vec::with_capacity(capacity),
            count: 0,
        }
    }

    // Insert an element while maintaining sorted order
    fn insert(&mut self, key: i32) -> usize {
        // Check if the element already exists
        if self.elements.binary_search(&key).is_ok() {
            return self.count; // Ignore insertion if it exists
        }
        
        // Find the position to insert the new element
        let pos = self.elements.iter().position(|&x| x > key).unwrap_or(self.count);
        self.elements.insert(pos, key);
        self.count += 1; // Increment count after successful insertion
        self.count // Return the current count
    }

    // Delete an element if it exists
    fn delete(&mut self, key: i32) {
        if let Ok(pos) = self.elements.binary_search(&key) {
            self.elements.remove(pos);
            self.count -= 1; // Decrement count after successful deletion
        }
        // If the element does not exist, do nothing (silent operation)
    }

    // Search for an element and return 1 if found, 0 otherwise
    fn search(&self, key: i32) -> i32 {
        if self.elements.binary_search(&key).is_ok() {
            1 // Element found
        } else {
            0 // Element not found
        }
    }

    // Dump elements within a specified range
    fn dump(&self, lower: i32, upper: i32) {
        for &element in &self.elements {
            if element >= lower && element <= upper {
                println!("{}", element); // Print each element in the range
            }
        }
    }
}

// Main function to demonstrate the usage of DynamicArray
fn main() {
    let mut dynamic_array = DynamicArray::new(10); // Initialize with capacity for 10 operations

    // Example operations (these would typically come from user input)
    dynamic_array.insert(5);
    dynamic_array.insert(3);
    dynamic_array.insert(8);
    dynamic_array.delete(3);
    let search_result = dynamic_array.search(5);
    dynamic_array.dump(1, 10);

    // Output the result of the search operation
    println!("Search result for 5: {}", search_result);
}
