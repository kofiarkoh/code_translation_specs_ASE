use std::vec::Vec;

struct DynamicArray {
    elements: Vec<i32>,
    count: usize,
    capacity: usize,
}

impl DynamicArray {
    fn new(size: usize) -> Self {
        DynamicArray {
            elements: Vec::with_capacity(size),
            count: 0,
            capacity: size,
        }
    }

    fn insert(&mut self, x: i32) -> usize {
        // Check for duplicates and capacity
        if self.count < self.capacity && !self.elements.contains(&x) {
            // Find the position to insert while maintaining sorted order
            let pos = self.elements.binary_search(&x).err().unwrap();
            self.elements.insert(pos, x);
            self.count += 1;
        }
        self.count
    }

    fn find(&self, x: i32) -> i32 {
        if self.elements.binary_search(&x).is_ok() {
            1 // Element exists
        } else {
            0 // Element does not exist
        }
    }

    fn delete(&mut self, x: i32) -> i32 {
        if let Ok(pos) = self.elements.binary_search(&x) {
            self.elements.remove(pos);
            self.count -= 1;
            1 // Deletion successful
        } else {
            -1 // Element not found
        }
    }

    fn dump(&self, x: i32, y: i32) {
        for &element in &self.elements {
            if element >= x && element <= y {
                println!("{}", element);
            }
        }
    }
}

fn main() {
    // Example usage
    let mut dynamic_array = DynamicArray::new(10);
    dynamic_array.insert(5);
    dynamic_array.insert(3);
    dynamic_array.insert(8);
    dynamic_array.dump(2, 6); // Should print 3 and 5
    println!("{}", dynamic_array.find(5)); // Should print 1
    println!("{}", dynamic_array.delete(5)); // Should print 1
    println!("{}", dynamic_array.find(5)); // Should print 0
}
