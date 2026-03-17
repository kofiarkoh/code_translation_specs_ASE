struct DynamicArray {
    elements: Vec<i32>,
}

impl DynamicArray {
    fn new(size: usize) -> Self {
        // Initialize the dynamic array with a specified size
        DynamicArray {
            elements: Vec::with_capacity(size),
        }
    }

    fn insert(&mut self, key: i32) -> usize {
        // Insert the key if it does not already exist
        if !self.elements.contains(&key) {
            // Find the position to insert while maintaining sorted order
            let pos = self.elements.binary_search(&key).err().unwrap();
            self.elements.insert(pos, key);
        }
        // Return the current count of elements
        self.elements.len()
    }

    fn delete(&mut self, key: i32) -> usize {
        // Remove the key if it exists
        if let Ok(pos) = self.elements.binary_search(&key) {
            self.elements.remove(pos);
        }
        // Return the current count of elements
        self.elements.len()
    }

    fn search(&self, key: i32) -> i32 {
        // Return 1 if the key exists, otherwise return 0
        if self.elements.binary_search(&key).is_ok() {
            1
        } else {
            0
        }
    }

    fn dump(&self, range_start: i32, range_end: i32) {
        // Output elements within the specified range
        for &element in &self.elements {
            if element >= range_start && element <= range_end {
                println!("{}", element);
            }
        }
    }
}

fn main() {
    // Example usage of the DynamicArray
    let mut dynamic_array = DynamicArray::new(10);
    
    // Simulated operations
    dynamic_array.insert(5);
    dynamic_array.insert(3);
    dynamic_array.insert(8);
    dynamic_array.delete(3);
    let exists = dynamic_array.search(5);
    dynamic_array.dump(1, 10);
    
    // Output the existence of the key
    println!("Key exists: {}", exists);
}
